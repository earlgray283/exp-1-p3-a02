package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Tag struct {
	Tag string
	Ids []uint64
}

func LoadTags(name string) ([]*Tag, error) {
	f, err := os.Open(name)
	if err != nil {
		return nil, err
	}
	sc := bufio.NewScanner(f)
	buf := make([]byte, 4830928)
	sc.Buffer(buf, 4830928)
	tags := []*Tag{}
	for sc.Scan() {
		cols := strings.Split(sc.Text(), ",")
		ids := make([]uint64, 0)
		for _, col := range cols[1:] {
			id, _ := strconv.ParseUint(col, 10, 64)
			ids = append(ids, id)
		}
		tags = append(tags, &Tag{
			Tag: cols[0],
			Ids: ids,
		})
	}
	return tags, nil
}

func FindGeotagIdsByTagName(tags []*Tag, tagName string) ([]uint64, error) {
	low, high := 0, len(tags)
	for low != high {
		mid := (low + high) / 2
		if tags[mid].Tag < tagName {
			low = mid + 1
		} else {
			high = mid
		}
	}

	if tags[low].Tag == tagName {
		return tags[low].Ids, nil
	} else {
		return nil, fmt.Errorf("tagName %s was not found", tagName)
	}
}
