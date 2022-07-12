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
	id  uint64
	tag string
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
	tags := make([]*Tag, 0)
	tagFile, _ := os.Open("csv/tag.csv")
	tagsc := bufio.NewScanner(tagFile)
	for tagsc.Scan() {
		tokens := strings.Split(strings.TrimSpace(tagsc.Text()), ",")
		id, _ := strconv.ParseUint(tokens[0], 10, 64)
		tags = append(tags, &Tag{
			id:  id,
			tag: tokens[1],
		})
	}
	tagFile.Close()
	sort.Slice(tags, func(i, j int) bool {
		return tags[i].id < tags[j].id
	})
	newTagFile, _ := os.Create("csv/new_tag.csv")
	newTagCsv := csv.NewWriter(newTagFile)
	for _, tag := range tags {
		cols := []string{strconv.FormatUint(tag.id, 10), tag.tag}
		newTagCsv.Write(cols)
	}
	newTagCsv.Flush()
	newTagFile.Close()

	geotags := make([]*Geotag, 0)
	geotagFile, _ := os.Open("csv/geotag.csv")
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
		return geotags[i].id < tags[j].id
	})

	newGeotagFile, _ := os.Create("csv/new_geotag.csv")
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
}
