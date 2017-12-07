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
			go handleShow(show, val, config, &wg)
		} else {
			fmt.Printf("%v: %v ✖\n", i+1, show.Title)
		}
	}

	wg.Wait()
	fmt.Println("Completed search")
}

func handleShow(show Series, provider Provider, config Config, wg *sync.WaitGroup) {
	defer wg.Done()

	handle := GetProvider(provider.BaseProvider)

	for i := 1; i < 4; i++ {
		err := handle(show, provider, config)
		if err != nil {
			fmt.Printf("Error in %v: %v\n", show.Title, err)
			fmt.Printf("Retry search for %v #%v\n", show.Title, i)
		} else {
			break
		}
	}

	fmt.Printf("Completed search for %v\n", show.Title)
}
