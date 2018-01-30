package fetcher

import "encoding/json"
import "io/ioutil"

func Parse(path string) (Config, error) {
	var config Config

	bytes, err := ioutil.ReadFile(path)
	if err != nil {
		return config, err
	}

	err = json.Unmarshal(bytes, &config)

	if err != nil {
		return config, err
	}

	return config, nil
}

type Config struct {
	WebUI WebUIConfig `json:"webui"`

	Api             string             `json:"api"`
	TransmissionRpc string             `json:"transmission_rpc"`
	Transmission    TransmissionConfig `json:"transmission"`
}

type TransmissionConfig struct {
	Rpc      string `json:"rpc"`
	User     string `json:"user"`
	Password string `json:"password"`
}

type WebUIConfig struct {
	Enable bool   `json:"enable"`
	Host   string `json:"host"`
}
