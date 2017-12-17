package fetcher

import (
	log "github.com/sirupsen/logrus"
	"sync"
)

func Fetch(config Config) {

	client := Init(config.Api)

	series, err := client.GetSeries()
	if err != nil {
		log.Printf("Error while getting series list: %v", err)
		return
	}

	supportedProviders, err := GetSupportedProviders(client)
	if err != nil {
		log.Printf("Error while getting supported providers: %v", err)
		return
	}

	var wg sync.WaitGroup

	for i, show := range series {

		if val, ok := supportedProviders[show.ProviderID]; ok {
			wg.Add(1)

			log.Printf("%v: %v ✓", i+1, show.Title)
			go handleShow(show, val, config, &wg)
		} else {
			log.Printf("%v: %v ✖", i+1, show.Title)
		}
	}

	wg.Wait()
	log.Println("Completed search")
}

func handleShow(show Series, provider Provider, config Config, wg *sync.WaitGroup) {
	defer wg.Done()

	handle := GetProvider(provider.BaseProvider)

	for i := 1; i < 4; i++ {
		err := handle(show, provider, config)
		if err != nil {
			log.Printf("Error in %v: %v", show.Title, err)
			log.Printf("Retry search for %v #%v", show.Title, i)
		} else {
			break
		}
	}

	log.Printf("Completed search for %v", show.Title)
}
