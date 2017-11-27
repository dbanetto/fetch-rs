package main

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
		fmt.Printf("%v: %v ", i, show.Title)

		if supportedProviders[show.ProviderID] != "" {
			fmt.Println("✓")
			handleShow(show, supportedProviders[show.ProviderID], config.Fetch)
		} else {
			fmt.Println("✖")
		}
	}
}

func handleShow(show Series, provider string, config FetchConfig) {

	GetProvider(provider).fetch(show)
}
