package fetcher

import (
	fetchapi "gitlab.com/zyphrus/fetch-api-go"
)

type FetchProvider func(show fetchapi.Series, config Config) error

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
