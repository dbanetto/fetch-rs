package fetcher

import (
	log "github.com/sirupsen/logrus"
	fetchapi "gitlab.com/zyphrus/fetch-api-go"
	"sync"
)

func Fetch(config Config) error {

	client := fetchapi.Init(config.Api)

	series, err := client.GetSeries()
	if err != nil {
		log.Errorf("Error while getting series list: %v", err)
		return err
	}

	var wg sync.WaitGroup

	for _, show := range series {
		wg.Add(1)

		log.WithField("title", show.Title).WithField("id", show.ID).Printf("Starting search for %v", show.Title)
		go handleShow(show, config, &wg)
	}

	wg.Wait()
	log.Println("Completed search")
	return nil
}

func handleShow(show fetchapi.Series, config Config, wg *sync.WaitGroup) {
	defer wg.Done()

	handle := GetProvider("nyaa") // FIXME: hard coded

	for i := 1; i < 4; i++ {
		err := handle(show, config)
		if err != nil {
			log.Printf("Error in %v: %v", show.Title, err)
			log.Printf("Retry search for %v #%v", show.Title, i)
		} else {
			break
		}
	}

	log.Printf("Completed search for %v", show.Title)
}
