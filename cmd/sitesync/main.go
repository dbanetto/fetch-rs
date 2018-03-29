package main

import (
	"flag"
	"fmt"
	log "github.com/sirupsen/logrus"
	fetchapi "gitlab.com/zyphrus/fetch-api-go"
	sitesync "gitlab.com/zyphrus/sitesync"
	"sync"
)

func main() {
	options := cli()

	config, err := sitesync.ParseConfig(options.ConfigPath)
	if err != nil {
		log.Errorf("Error loading config %s", err)
		return
	}

	fmt.Printf("Config %s\n", config)

	fetch := fetchapi.Init(config.FetchAPI.URL)

	series, err := fetch.GetSeries()

	if err != nil {
		log.Errorf("error recieveing series: %v", err)
		return
	}

	var wg sync.WaitGroup

	for _, show := range series {
		wg.Add(1)

		go handleShow(config, show, fetch, &wg)
	}

	wg.Wait()
}

func handleShow(config sitesync.Config, show fetchapi.Series, api *fetchapi.API, wg *sync.WaitGroup) {
	defer wg.Done()

	blobs, err := api.GetInfoBlob(show.ID, []string{"count", "mal", "kitsu"})
	if err != nil {
		log.
			WithField("title", show.Title).
			WithField("id", show.ID).
			Errorf("error recieveing info blobs: %v", err)
		return
	}

	count, err := blobs.GetType("count")
	if err != nil {
		log.
			WithField("title", show.Title).
			WithField("id", show.ID).
			Warnf("count blob not present")
		return
	}

	mal, err := blobs.GetType("mal")
	if err != nil {
		log.
			WithField("title", show.Title).
			WithField("id", show.ID).
			Warnf("mal blob not present")
	} else {
		// sync to MAL
		err := sitesync.SyncMAL(config.Mal, count, mal)
		if err != nil {
			log.
				WithField("title", show.Title).
				WithField("id", show.ID).
				WithField("mal", mal.Blob).
				WithField("count", count.Blob).
				Errorf("error during MAL sync: %v", err)
		}
	}

	kitsu, err := blobs.GetType("kitsu")
	if err != nil {
		log.
			WithField("title", show.Title).
			WithField("id", show.ID).
			Warnf("kitsu blob not present")
	} else {
		// sync to kitsu
		err := sitesync.SyncKitsu(config.Mal, count, mal)
		if err != nil {
			log.
				WithField("title", show.Title).
				WithField("id", show.ID).
				WithField("kitsu", kitsu.Blob).
				WithField("count", count.Blob).
				Errorf("error during Kitsu sync: %v", err)
		}
	}

	log.
		WithField("title", show.Title).
		WithField("id", show.ID).
		Info("Successfully completed sync")
}

func cli() Options {

	config := flag.String("config", "config.toml", "Path to the configuration file.")

	flag.Parse()

	return Options{
		*config,
	}
}

type Options struct {
	ConfigPath string
}
