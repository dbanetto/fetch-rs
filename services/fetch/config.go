package fetcher

import (
	toml "github.com/pelletier/go-toml"
	"io/ioutil"
)

func Parse(path string) (Config, error) {
	var config Config

	bytes, err := ioutil.ReadFile(path)
	if err != nil {
		return config, err
	}

	err = toml.Unmarshal(bytes, &config)

	if err != nil {
		return config, err
	}

	return config, nil
}

type Config struct {
	WebUI        WebUIConfig `toml:"webui"`
	Log          LogConfig
	FetchApi     string
	Transmission TransmissionConfig
}

type TransmissionConfig struct {
	Rpc      string
	User     string
	Password string
}

type WebUIConfig struct {
	Enable bool
	Host   string
}

type LogConfig struct {
	ToJSON bool
}
