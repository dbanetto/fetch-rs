package sitesync

import (
	"github.com/pelletier/go-toml"
	"io/ioutil"
)

func ParseConfig(path string) (Config, error) {

	var config Config

	// load from file
	bytes, err := ioutil.ReadFile(path)
	if err != nil {
		return config, err
	}

	err = toml.Unmarshal(bytes, &config)

	return config, err
}

type Config struct {
	FetchAPI ApiConfig
	Kitsu    KitsuConfig
	Mal      SiteConfig
}

type SiteConfig struct {
	Username string
	Password string
}

type KitsuConfig struct {
	Username string
	Password string
	UserID   string
}

type ApiConfig struct {
	URL string
}
