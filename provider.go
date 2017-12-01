package fetcher

type FetchProvider func(show Series, provider Provider, config FetchConfig) error

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

func GetSupportedProviders(client *API) (map[int]Provider, error) {

	result := make(map[int]Provider)

	providers, err := client.GetProviders()
	if err != nil {
		return result, err
	}

	for _, provider := range providers {
		if baseProviders[provider.BaseProvider] != nil {
			result[provider.ID] = provider
		}
	}

	return result, nil
}
