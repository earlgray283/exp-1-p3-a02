package main

import (
	"bufio"
	"encoding/csv"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
)

type Tag struct {
	tag string
	ids []uint64
}

type Geotag struct {
	id        uint64
	elapsed   uint64
	latitude  float64
	longitude float64
	farmNum   int    // http://farm{farmNum}.flickr...
	directory string // /8237/8520927781_4f86a7a3b1.jpg
}

func main() {
	if err := processTagCsv("csv/tag.csv"); err != nil {
		log.Fatal(err)
	}
	if err := processGeotagCsv("csv/geotag.csv"); err != nil {
		log.Fatal(err)
	}
}

func processTagCsv(name string) error {
	tagFile, err := os.Open(name)
	if err != nil {
		return err
	}
	tagsc := bufio.NewScanner(tagFile)
	tagmap := map[string][]uint64{}
	for tagsc.Scan() {
		tokens := strings.Split(strings.TrimSpace(tagsc.Text()), ",")
		id, _ := strconv.ParseUint(tokens[0], 10, 64)
		tag := tokens[1]
		if tag == "" {
			continue
		}
		if _, ok := tagmap[tag]; !ok {
			tagmap[tag] = make([]uint64, 0)
		}
		tagmap[tag] = append(tagmap[tag], id)
	}
	tagFile.Close()
	tags := make([]*Tag, 0, len(tagmap))
	for tag, ids := range tagmap {
		tags = append(tags, &Tag{tag, ids})
	}

	sort.Slice(tags, func(i, j int) bool {
		return tags[i].tag < tags[j].tag
	})

	newTagFile, err := os.Create("csv/new_tag.csv")
	if err != nil {
		return err
	}
	newTagCsv := csv.NewWriter(newTagFile)
	for _, tag := range tags {
		cols := []string{tag.tag}
		for _, id := range tag.ids {
			cols = append(cols, strconv.FormatUint(id, 10))
		}
		newTagCsv.Write(cols)
	}
	newTagCsv.Flush()
	newTagFile.Close()

	return nil
}

func processGeotagCsv(name string) error {
	geotagFile, err := os.Open(name)
	if err != nil {
		return err
	}
	geotags := make([]*Geotag, 0)
	geotagsc := bufio.NewScanner(geotagFile)
	for geotagsc.Scan() {
		tokens := strings.Split(strings.TrimSpace(geotagsc.Text()), ",")
		id, _ := strconv.ParseUint(tokens[0], 10, 64)
		baseDate := time.Date(2012, time.January, 1, 0, 0, 0, 0, time.Local)
		date, err := time.Parse("2006-01-02 15:04:05", strings.Trim(tokens[1], "\""))
		if err != nil {
			log.Fatal(err)
		}
		latitude, _ := strconv.ParseFloat(tokens[2], 64)
		longitude, _ := strconv.ParseFloat(tokens[3], 64)
		var farmNum int
		var directory string
		fmt.Sscanf(tokens[4], "http://farm%d.static.flickr.com%s", &farmNum, &directory)
		geotags = append(geotags, &Geotag{
			id:        id,
			elapsed:   uint64(date.Sub(baseDate).Seconds()),
			latitude:  latitude,
			longitude: longitude,
			farmNum:   farmNum,
			directory: directory,
		})
	}
	geotagFile.Close()

	sort.Slice(geotags, func(i, j int) bool {
		return geotags[i].id < geotags[j].id
	})

	newGeotagFile, err := os.Create("csv/new_geotag.csv")
	if err != nil {
		return err
	}
	newGeotagCsv := csv.NewWriter(newGeotagFile)
	for _, geotag := range geotags {
		cols := []string{
			strconv.FormatUint(geotag.id, 10),
			strconv.FormatUint(geotag.elapsed, 10),
			strconv.FormatFloat(geotag.latitude, 'f', -1, 64),
			strconv.FormatFloat(geotag.longitude, 'f', -1, 64),
			strconv.FormatUint(uint64(geotag.farmNum), 10),
			geotag.directory,
		}
		newGeotagCsv.Write(cols)
	}
	newGeotagCsv.Flush()
	newGeotagFile.Close()

	return nil
}
