package fetcher

import (
	"crypto/tls"
	"github.com/odwrtw/transmission"
	"net/http"
	"strings"
)

func buildTransmission(config Config) (*transmission.Client, error) {
	tr := &http.Transport{
		TLSClientConfig: &tls.Config{InsecureSkipVerify: true},
	}
	httpClient := http.Client{Transport: tr}
	conf := transmission.Config{
		Address:    config.TransmissionRpc,
		HTTPClient: &httpClient,
	}

	return transmission.New(conf)
}

func resolveSearchTitle(show Series) string {
	// Resolve the search title ti use
	// by default use the title
	search_title := strings.TrimSpace(show.Title)
	if show.SearchTitle != "" {
		// otherwise if 'search_title' is defined use that
		search := strings.TrimSpace(show.SearchTitle)
		if search != "" {
			search_title = search
		}
	}

	return search_title
}
