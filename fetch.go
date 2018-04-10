package fetcher

import (
	log "github.com/sirupsen/logrus"
	fetchapi "gitlab.com/zyphrus/fetch-api-go"
	"sync"
)

func Fetch(config Config) error {

	client := fetchapi.Init(config.FetchApi)

	series, err := client.GetSeries()
	if err != nil {
		log.WithField("err", err).Error("Error while getting series list")
		return err
	}

	var wg sync.WaitGroup

	for _, show := range series {
		wg.Add(1)

		log.
			WithField("title", show.Title).
			WithField("id", show.ID).
			Info("Starting search")
		go handleShow(show, config, &wg)
	}

	wg.Wait()
	log.Info("Completed search")
	return nil
}

func handleShow(show fetchapi.Series, config Config, wg *sync.WaitGroup) {
	defer wg.Done()

	handle := GetProvider("nyaa") // FIXME: hard coded

	for i := 1; i < 4; i++ {
		err := handle(show, config)
		if err != nil {
			log.
				WithField("try", i).
				WithField("title", show.Title).
				Warn("Retry search")
		} else {
			break
		}
	}
}
