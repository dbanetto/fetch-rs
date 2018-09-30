package fetcher

import (
	log "github.com/sirupsen/logrus"
	fetchapi "gitlab.com/zyphrus/fetch-api-go"
)

func Fetch(config Config) ([]FetchResult, error) {

	client := fetchapi.Init(config.FetchApi)

	series, err := client.GetSeries()
	if err != nil {
		log.WithField("err", err).Error("Error while getting series list")
		return nil, err
	}

    channels := make([]chan FetchResult, len(series))
    results := make([]FetchResult, len(series))

	for n, show := range series {
        chann := make(chan FetchResult)
        channels[n] = chann

		log.
			WithField("title", show.Title).
			WithField("id", show.ID).
			Info("Starting search")
		go handleShow(show, config, chann)
	}

    log.Info("Waiting for responses from channels")
    for n, chann := range channels {
        result := <-chann
        results[n] = result
    }

	log.Info("Completed search")
	return results, nil
}

func handleShow(show fetchapi.Series, config Config, chann chan FetchResult) {
	handle := GetProvider("nyaa") // FIXME: hard coded

	var result FetchResult
	for i := 1; i < 4; i++ {
		res, err := handle(show, config)
		result = res
		if err != nil {
			log.
				WithField("try", i).
				WithField("title", show.Title).
				Warn("Retry search")
		} else {
			break
		}
	}

	chann <- result
}
