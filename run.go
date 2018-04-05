package sitesync

import (
	log "github.com/sirupsen/logrus"
	fetchapi "gitlab.com/zyphrus/fetch-api-go"
	"sync"
)

func Run(config Config) error {

	fetch := fetchapi.Init(config.FetchAPI.URL)

	series, err := fetch.GetSeries()

	if err != nil {
		log.Errorf("error recieveing series: %v", err)
		return err
	}

	malCreds := &config.Mal
	err = CheckMALCreds(&config.Mal)
	if err != nil {
		log.
			WithField("err", err).
			WithField("mal_user", config.Mal.Username).
			Errorf("Could not verify credentials for MAL")
		return err
	}

	kitsuSession, err := GetKitsuToken(config.Kitsu)
	kitsuToken := &kitsuSession
	if err != nil {
		log.
			WithField("err", err).
			WithField("kitsu_user", config.Kitsu.Username).
			Errorf("Could get Kitsu token: %v", err)
		return err
	}

	var wg sync.WaitGroup

	for _, show := range series {
		wg.Add(1)

		go handleShow(malCreds, kitsuToken, show, fetch, &wg)
	}

	wg.Wait()
	return nil
}

func handleShow(
	malCred *SiteConfig,
	kitsuSession *KitsuSession,
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

	if malCred != nil {
		mal, err := blobs.GetType("mal")
		if err != nil {
			logTitle.Warnf("mal blob not present")
		} else {
			// sync to MAL
			err := SyncMAL(logTitle, malCred, count, mal)
			if err != nil {
				logTitle.
					WithField("mal", mal.Blob).
					WithField("count", count.Blob).
					Errorf("error during MAL sync: %v", err)
			}
		}
	} else {
		logTitle.Warn("Skipped MAL due to lack of valid credentials")
	}

	if kitsuSession != nil {
		kitsu, err := blobs.GetType("kitsu")
		if err != nil {
			logTitle.Warnf("kitsu blob not present")
		} else {
			// sync to kitsu
			logTitle.
				WithField("kitsu", kitsu.Blob).
				WithField("count", count.Blob).
				Info("starting to sync")
			err := SyncKitsu(logTitle, kitsuSession, count, kitsu)
			if err != nil {
				logTitle.
					WithField("kitsu", kitsu.Blob).
					WithField("count", count.Blob).
					Errorf("error during Kitsu sync: %v", err)
			}
		}
	} else {
		logTitle.Warn("Skipped Kitsu due to lack of valid credentials")
	}

	logTitle.Info("Completed")
}
