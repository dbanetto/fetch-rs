package fetcher

import (
	fetchapi "gitlab.com/zyphrus/fetch-api-go"
)

type FetchResult struct {
    ID int `json:"id"`;
    Success bool `json:"success"`;
    Found bool `json:"found"`;
    Count int `json:"count"`;
}

type FetchProvider func(show fetchapi.Series, config Config) (FetchResult, error)

var baseProviders map[string]FetchProvider

func GetProvider(name string) FetchProvider {
	return baseProviders[name]
}

func RegisterFetchProvider(name string, provider FetchProvider) {
	if baseProviders == nil {
		baseProviders = make(map[string]FetchProvider)
	}

	baseProviders[name] = provider
}
