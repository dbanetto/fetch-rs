package main

type FetchProvider interface {
	fetch(show Series) error
}

var baseProviders map[string]FetchProvider

func GetProvider(name string) FetchProvider {
	return baseProviders[name]
}

func RegisterFetchProvider(name string, provider FetchProvider) {
	baseProviders[name] = provider
}

func GetSupportedProviders(client *API) (map[int]string, error) {

	result := make(map[int]string)

	providers, err := client.GetProviders()
	if err != nil {
		return result, err
	}

	for _, provider := range providers {
		if baseProviders[provider.BaseProvider] != nil {
			result[provider.ID] = provider.BaseProvider
		}
	}

	return result, nil
}
