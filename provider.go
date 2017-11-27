package main

type FetchProvider interface {
	fetch()
}

var baseProviders map[string]FetchProvider

func RegisterFetchProvider(name string, provider FetchProvider) {
	baseProviders[name] = provider
}

func GetSupportedProviders(client *API) (map[int]bool, error) {

	result := make(map[int]bool)

	providers, err := client.GetProviders()
	if err != nil {
		return result, err
	}

	for _, provider := range providers {
		result[provider.ID] = baseProviders[provider.BaseProvider] != nil
	}

	return result, nil
}
