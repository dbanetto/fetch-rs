package fetcher

import (
	"crypto/tls"
	"encoding/xml"
	"fmt"
	"github.com/odwrtw/transmission"
	log "github.com/sirupsen/logrus"
	"io/ioutil"
	"net/http"
	"net/url"
	"regexp"
	"strconv"
	"strings"
)

func NyaaFetch(show Series, provider Provider, config Config) error {

	// type def for later
	type Nyaa struct {
		Items []struct {
			Title string `xml:"title"`
			Link  string `xml:"link"`
		} `xml:"channel>item"`
	}

	tr := &http.Transport{
		TLSClientConfig: &tls.Config{InsecureSkipVerify: true},
	}
	httpClient := http.Client{Transport: tr}
	conf := transmission.Config{
		Address:    config.TransmissionRpc,
		HTTPClient: &httpClient,
	}

	t, err := transmission.New(conf)
	if err != nil {
		return nil
	}

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

	// Supported  media type option
	quality := url.PathEscape(show.MediaTypeOptions["quality"])

	// build the url
	search_url := fmt.Sprintf("https://www.nyaa.si/?page=rss&user=%v&term=%v+%v",
		provider.BaseProviderOptions["id"],
		quality,
		url.QueryEscape(search_title))

	// logs the resulting URL
	log.Infof("Searching with %v", search_url)

	// Build the request
	client := &http.Client{}
	req, err := http.NewRequest("GET", search_url, nil)
	if err != nil {
		log.Errorf("Error during request building")
		return err
	}
	req.Close = true

	// Handle the response
	resp, err := client.Do(req)
	if err != nil {
		log.Errorf("Error during request")
		return err
	}
	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil { // FIXME: this can fail with EOF due to unknown reasons
		log.Errorf("Error during read")
		return err
	}

	// Parse XML into  struct
	res := new(Nyaa)
	err = xml.Unmarshal(body, &res)
	if err != nil {
		log.Errorf("Error during Unmarshal")
		return err
	}

	maxCount := show.CurrentCount

	// Use found items in XML
	for _, item := range res.Items {
		// strip out title to ensure no missmatches
		epi := strings.Replace(item.Title, search_title, "", 1)

		// find count using given regexp
		find_count := regexp.MustCompile(provider.RegexFindCount)
		count_match := strings.TrimLeft(find_count.FindString(epi), "0")

		count, err := strconv.Atoi(count_match)
		if err != nil {
			log.Errorf("ERROR parsing episode count (%v): %v", count_match, err)
		}

		// check if this is a new episode found
		if count > show.CurrentCount {
			log.Infof("Found episode %v of %v", count, show.Title)

			// push episode to transmission
			_, err := t.Add(item.Link)
			if err != nil {
				log.Errorf("ERROR while pushing url to transmission (%v): %v", item.Link, err)
			}

			if count > maxCount {
				maxCount = count
			}
		}
	}

	if maxCount > show.CurrentCount {
		// push update to API
		log.Infof("Update episode of %v to %v", show.Title, maxCount)
		api := Init(config.Api)
		return api.PostEpisodeCount(show.ID, maxCount)
	}

	// everything must of been ffiinnee
	return nil
}
