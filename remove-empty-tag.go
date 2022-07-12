package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type Tag struct {
	id  uint64
	tag string
}

type Geotag struct {
	id        uint64
	date      string
	latitude  float64
	longitude float64
	url       string
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

	geotags := make([]*Geotag, 0)
	geotagFile, _ := os.Open("csv/geotag.csv")
	geotagsc := bufio.NewScanner(geotagFile)
	for geotagsc.Scan() {
		tokens := strings.Split(strings.TrimSpace(geotagsc.Text()), ",")
		id, _ := strconv.ParseUint(tokens[0], 10, 64)
		latitude, _ := strconv.ParseFloat(tokens[2], 64)
		longitude, _ := strconv.ParseFloat(tokens[3], 64)
		geotags = append(geotags, &Geotag{
			id:        id,
			date:      tokens[1],
			latitude:  latitude,
			longitude: longitude,
			url:       tokens[4],
		})
	}
	geotagFile.Close()

	cnt := 0
	for _, tag := range tags {
		if tag.tag == "" {
			cnt++
		}
	}
	fmt.Println(cnt)
}
