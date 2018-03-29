package main

import (
	"flag"
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

	fetch := fetchapi.Init(config.FetchAPI.URL)

	series, err := fetch.GetSeries()

	if err != nil {
		log.Errorf("error recieveing series: %v", err)
		return
	}

	err = sitesync.CheckMALCreds(config.Mal)

	kitsuSession, err := sitesync.GetKitsuToken(config.Kitsu)
	if err != nil {
		log.
			WithField("err", err).
			WithField("kitsu_user", config.Kitsu.Username).
			Errorf("Could get Kitsu token: %v", err)
	}

	var wg sync.WaitGroup

	for _, show := range series {
		wg.Add(1)

		go handleShow(config.Mal, kitsuSession, show, fetch, &wg)
	}

	wg.Wait()
}

func handleShow(
	malCred sitesync.SiteConfig,
	kitsuSession sitesync.KitsuSession,
	show fetchapi.Series,
	api *fetchapi.API,
	wg *sync.WaitGroup) {

	defer wg.Done()

	blobs, err := api.GetInfoBlob(show.ID, []string{"count", "mal", "kitsu"})
	logTitle := log.WithField("title", show.Title).WithField("id", show.ID)

	if err != nil {
		logTitle.Errorf("error recieveing info blobs: %v", err)
		return
	}

	count, err := blobs.GetType("count")
	if err != nil {
		logTitle.Warnf("count blob not present")
		return
	}

	mal, err := blobs.GetType("mal")
	if err != nil {
		logTitle.Warnf("mal blob not present")
	} else {
		// sync to MAL
		err := sitesync.SyncMAL(logTitle, malCred, count, mal)
		if err != nil {
			logTitle.
				WithField("mal", mal.Blob).
				WithField("count", count.Blob).
				Errorf("error during MAL sync: %v", err)
		}
	}

	kitsu, err := blobs.GetType("kitsu")
	if err != nil {
		logTitle.Warnf("kitsu blob not present")
	} else {
		// sync to kitsu
		err := sitesync.SyncKitsu(logTitle, kitsuSession, count, mal)
		if err != nil {
			logTitle.
				WithField("kitsu", kitsu.Blob).
				WithField("count", count.Blob).
				Errorf("error during Kitsu sync: %v", err)
		}
	}

	logTitle.Info("Successfully completed sync")
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
