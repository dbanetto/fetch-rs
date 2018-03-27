package main

import (
	"flag"
	"fmt"
	"gitlab.com/zyphrus/sitesync"
)

func main() {
	options := cli()

	config, err := sitesync.ParseConfig(options.ConfigPath)
	if err != nil {
		fmt.Printf("Error loading config %s", err)
	}

	fmt.Printf("Config %s\n", config)

}

func cli() Options {

	config := flag.String("config", "config.toml", "Path to the configuration file.")

	flag.Parse()

	return Options{
		*config,
	}
}

type Options struct {
	ConfigPath string
}
