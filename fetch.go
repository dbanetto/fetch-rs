package fetcher

import "fmt"

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

	for i, show := range series {
		fmt.Printf("%v: %v ", i+1, show.Title)

		if val, ok := supportedProviders[show.ProviderID]; ok {
			fmt.Println("✓")
			handleShow(show, val, config.Fetch)
		} else {
			fmt.Println("✖")
		}
	}
}

func handleShow(show Series, provider Provider, config FetchConfig) {

	handle := GetProvider(provider.BaseProvider)

	err := handle(show, provider, config)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	}

}
