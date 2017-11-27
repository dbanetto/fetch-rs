package main

import "fmt"

func Fetch(config Config) {

	client := Init(config.Api)

	series, err := client.GetSeries()

	if err != nil {
		fmt.Errorf("Error during series %v", err)
		return
	}

	for i, show := range series {
		fmt.Printf("%v: %v\n", i, show.Title)
		handleShow(show, config.Fetch)
	}

}

func handleShow(show Series, config FetchConfig) {

}
