package main

import "github.com/zyphrus/fetcher"
import "fmt"
import "flag"

func main() {

	options := cli()

	config, err := fetcher.Parse(options.ConfigPath)

	if err != nil {
		fmt.Errorf("Error while loading config: %v\n", err)
		return
	}

	fmt.Println(options)
	fmt.Println(config)

	if options.Fetch {
		fetcher.Fetch(config)
	} else if options.Sort {
		// TODO
	} else if config.WebUI.Enable {
		// default action to start web server
		fetcher.StartWeb(config)
	}
}

func cli() Options {

	sort := flag.Bool("sort", false, "Force sort")
	fetch := flag.Bool("fetch", false, "Force fetch")
	config := flag.String("config", "config.json", "Path to configuration file")

	flag.Parse()

	return Options{
		*sort,
		*fetch,
		*config,
	}
}

type Options struct {
	Sort       bool
	Fetch      bool
	ConfigPath string
}
