package fetcher

type FetchProvider func(show Series, config Config) error

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
