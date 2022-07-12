package main

import (
	"bufio"
	"errors"
	"os"
	"strconv"
	"strings"
)

type Geotag struct {
	Id        uint64
	Elapsed   uint32
	Latitude  float32
	Longitude float32
	FarmNum   int8
	Directory string
}

func LoadGeotags(name string) ([]*Geotag, error) {
	f, err := os.Open(name)
	if err != nil {
		return nil, err
	}
	sc := bufio.NewScanner(f)
	geotags := []*Geotag{}
	for sc.Scan() {
		cols := strings.Split(sc.Text(), ",")
		id, _ := strconv.ParseUint(cols[0], 10, 64)
		elapsed, _ := strconv.ParseUint(cols[1], 10, 32)
		latitude, _ := strconv.ParseFloat(cols[2], 32)
		longitude, _ := strconv.ParseFloat(cols[3], 32)
		farmNum, _ := strconv.ParseInt(cols[4], 10, 8)
		directory := cols[5]
		geotags = append(geotags, &Geotag{
			Id:        id,
			Elapsed:   uint32(elapsed),
			Latitude:  float32(latitude),
			Longitude: float32(longitude),
			FarmNum:   int8(farmNum),
			Directory: directory,
		})
	}
	return geotags, nil
}

func FindGeotagById(geotags []*Geotag, id uint64) (*Geotag, error) {
	low, high := 0, len(geotags)
	for low != high {
		mid := (low + high) / 2
		if geotags[mid].Id < id {
			low = mid + 1
		} else {
			high = mid
		}
	}
	if geotags[low].Id == id {
		return geotags[low], nil
	} else {
		return nil, errors.New("")
	}
}
