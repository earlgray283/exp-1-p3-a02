package main

import (
	"fmt"
	"log"
	"net/http"
	"sort"
	"strings"
	"sync"
	"time"
)

const SubtagsLimit = 100

type Pair[T, U any] struct {
	First  T
	Second U
}

func main() {
	tagResCh := make(chan *Pair[[]*Tag, error])
	go func() {
		tags, err := LoadTags("csv/new_tag.csv")
		if err != nil {
			tagResCh <- &Pair[[]*Tag, error]{nil, err}
		}
		fmt.Println(len(tags))
		sort.Slice(tags, func(i, j int) bool {
			return tags[i].Tag < tags[j].Tag
		})
		tagResCh <- &Pair[[]*Tag, error]{tags, nil}
	}()
	geotagResCh := make(chan *Pair[[]*Geotag, error])
	go func() {
		geotags, err := LoadGeotags("csv/new_geotag.csv")
		if err != nil {
			geotagResCh <- &Pair[[]*Geotag, error]{nil, err}
		}
		fmt.Println(len(geotags))
		sort.Slice(geotags, func(i, j int) bool {
			return geotags[i].Id < geotags[j].Id
		})
		geotagResCh <- &Pair[[]*Geotag, error]{geotags, nil}
	}()
	tagRes := <-tagResCh
	if tagRes.Second != nil {
		log.Fatal(tagRes.Second)
	}
	tags := tagRes.First
	geotagRes := <-geotagResCh
	if geotagRes.Second != nil {
		log.Fatal(geotagRes.Second)
	}
	geotags := geotagRes.First

	http.HandleFunc("/program", handleSearchTag(tags, geotags))

	fmt.Println("Listening at http://localhost:8080...")
	if err := http.ListenAndServe(":8080", nil); err != nil {
		log.Fatal(err)
	}
}

func handleSearchTag(tags []*Tag, geotags []*Geotag) http.HandlerFunc {
	return func(rw http.ResponseWriter, r *http.Request) {
		tag := r.URL.Query().Get("tag")

		subtags, err := FindGeotagIdsByTagName(tags, tag)
		if err != nil {
			log.Println(err)
			rw.WriteHeader(http.StatusNotFound)
			return
		}

		mu := &sync.Mutex{}
		subgeotags := make([]*Geotag, 0, len(subtags))
		wg := &sync.WaitGroup{}
		for _, subtag := range subtags {
			wg.Add(1)
			go func(subtag uint64) {
				geotag, err := FindGeotagById(geotags, subtag)
				if err != nil {
					log.Println(err)
					rw.WriteHeader(http.StatusNotFound)
					return
				}
				mu.Lock()
				subgeotags = append(subgeotags, geotag)
				mu.Unlock()
				wg.Done()
			}(subtag)
		}
		wg.Wait()

		sort.Slice(subgeotags, func(i, j int) bool {
			return subgeotags[i].Elapsed > subgeotags[j].Elapsed
		})

		baseDate := time.Date(2012, 1, 1, 0, 0, 0, 0, time.Local)

		htmlBuilder := &strings.Builder{}
		htmlBuilder.WriteString("<!DOCTYPE html>\n")
		htmlBuilder.WriteString("<html>\n")
		htmlBuilder.WriteString("<head>\n")
		htmlBuilder.WriteString("<meta charset=\"utf-8\">\n")
		htmlBuilder.WriteString("<title>実装A(Go)の結果</title>\n")
		htmlBuilder.WriteString("</head>\n")
		htmlBuilder.WriteString("<body>\n")
		htmlBuilder.WriteString(fmt.Sprintf("<h1>%s</h1>\n", tag))
		htmlBuilder.WriteString("<table>\n")
		htmlBuilder.WriteString("<tr>\n")
		htmlBuilder.WriteString("<th>id</th>\n")
		htmlBuilder.WriteString("<th>latitude</th>\n")
		htmlBuilder.WriteString("<th>longitude</th>\n")
		htmlBuilder.WriteString("<th>date</th>\n")
		htmlBuilder.WriteString("</tr>\n")
		subgeotagsLimit := SubtagsLimit
		if len(subgeotags) < SubtagsLimit {
			subgeotagsLimit = len(subgeotags)
		}
		for _, geotag := range subgeotags[:subgeotagsLimit] {
			htmlBuilder.WriteString("<tr>\n")
			htmlBuilder.WriteString(fmt.Sprintf("<td>%d</td>\n", geotag.Id))
			htmlBuilder.WriteString(fmt.Sprintf("<td>%f</td>\n", geotag.Latitude))
			htmlBuilder.WriteString(fmt.Sprintf("<td>%f</td>\n", geotag.Longitude))
			htmlBuilder.WriteString(fmt.Sprintf("<td>%s</td>\n", baseDate.Add(time.Duration(geotag.Elapsed)*time.Second).Format("2006-01-02 15:04:05")))
			htmlBuilder.WriteString("</tr>\n")
		}
		htmlBuilder.WriteString("</table>\n")
		htmlBuilder.WriteString("</body>\n")
		htmlBuilder.WriteString("</html>\n")

		rw.WriteHeader(http.StatusOK)
		rw.Header().Set("Content-Type", "text/html")
		rw.Write([]byte(htmlBuilder.String()))
	}
}
