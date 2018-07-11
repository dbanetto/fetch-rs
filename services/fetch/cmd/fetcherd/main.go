package main

import (
	"fetch"
	"flag"
	log "github.com/sirupsen/logrus"
)

var version string

func main() {

	options := cli()

	config, err := fetcher.Parse(options.ConfigPath)

	fetcher.RegisterFetchProvider("nyaa", fetcher.NyaaFetch)
	// fetcher.RegisterFetchProvider("magnetrss", fetcher.MagnetRss)

	if err != nil {
		log.WithField("args", options).Fatal("Error while loading config:", err)
		return
	}

	if config.Log.ToJSON {
		log.SetFormatter(&log.JSONFormatter{})
	}

	// logout the version
	if version != "" {
		log.
			WithField("version", version).
			Info()
	} else {
		log.Warn("Unknown version")
	}

	log.WithFields(log.Fields{
		"config": config,
		"args":   options,
	}).Debug("Loaded settings")

	if options.Fetch {
		log.Info("Starting manual fetch")
		fetcher.Fetch(config)
	} else if config.WebUI.Enable {
		// default action to start web server
		fetcher.StartWeb(config)
	}
}

func cli() Options {

	fetch := flag.Bool("fetch", false, "Force fetch")
	config := flag.String("config", "config.toml", "Path to configuration file")

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
