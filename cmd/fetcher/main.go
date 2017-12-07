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
	} else if config.WebUI.Enable {
		// default action to start web server
		fetcher.StartWeb(config)
	}
}

func cli() Options {

	fetch := flag.Bool("fetch", false, "Force fetch")
	config := flag.String("config", "config.json", "Path to configuration file")

	flag.Parse()

	return Options{
		*fetch,
		*config,
	}
}

type Options struct {
	Fetch      bool
	ConfigPath string
}
