package fetcher

import "fmt"
import "sync"

func Fetch(config Config) {

	client := Init(config.Api)

	series, err := client.GetSeries()
	if err != nil {
		fmt.Errorf("Error while getting series list: %v", err)
		return
	}

	supportedProviders, err := GetSupportedProviders(client)
	if err != nil {
		fmt.Errorf("Error while getting supported providers: %v", err)
		return
	}

	var wg sync.WaitGroup

	for i, show := range series {

		if val, ok := supportedProviders[show.ProviderID]; ok {
			wg.Add(1)

			fmt.Printf("%v: %v ✓\n", i+1, show.Title)
			go handleShow(show, val, config.Fetch, &wg)
		} else {
			fmt.Printf("%v: %v ✖\n", i+1, show.Title)
		}
	}

	wg.Wait()
	fmt.Println("Completed search")
}

func handleShow(show Series, provider Provider, config FetchConfig, wg *sync.WaitGroup) {
	defer wg.Done()

	handle := GetProvider(provider.BaseProvider)

	err := handle(show, provider, config)
	if err != nil {
		fmt.Printf("Error in %v: %v\n", show.Title, err)
	}

	fmt.Printf("Completed search for %v\n", show.Title)
}
