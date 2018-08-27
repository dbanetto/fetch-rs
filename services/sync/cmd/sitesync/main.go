package main

import (
	"flag"
	log "github.com/sirupsen/logrus"
	sitesync "sitesync"
)

func main() {
	options := cli()

	config, err := sitesync.ParseConfig(options.ConfigPath)
	if err != nil {
		log.Errorf("Error loading config %s", err)
		return
	}

	if config.JSONLog {
		log.SetFormatter(&log.JSONFormatter{})
	}

	if options.Sync {
		sitesync.Run(config)
	} else {
		sitesync.StartWeb(config)
	}
}

func cli() Options {

	config := flag.String("config", "config.toml", "Path to the configuration file.")
	sync := flag.Bool("sync", false, "Manual synchronise.")

	flag.Parse()

	return Options{
		*sync,
		*config,
	}
}

type Options struct {
	Sync       bool
	ConfigPath string
}
