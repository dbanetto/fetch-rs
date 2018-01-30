package fetcher

import (
	"encoding/xml"
	log "github.com/sirupsen/logrus"
	"io/ioutil"
	"net/http"
	"regexp"
	"strconv"
	"strings"
)

func MagnetRss(show Series, provider Provider, config Config) error {

	// type def for later
	type Rss struct {
		Items []struct {
			Title string `xml:"title"`
			Link  string `xml:"link"`
		} `xml:"channel>item"`
	}

	t, err := buildTransmission(config)
	if err != nil {
		return nil
	}

	feedUrl := provider.BaseProviderOptions["feed_url"]

	if feedUrl == "" {
		log.WithField("series", show.Title).Error("Feed url is empty")
		return nil
	}
	log.WithField("url", feedUrl).Info("Feed url for ", show.Title)

	search_title := resolveSearchTitle(show)

	log.Info("Searching for ", search_title)

	// build the request
	client := &http.Client{}
	req, err := http.NewRequest("GET", feedUrl, nil)
	if err != nil {
		log.Error("Error during request building")
		return err
	}
	req.Close = true

	// Handle the response
	resp, err := client.Do(req)
	if err != nil {
		log.Error("Error during request")
		return err
	}
	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil { // FIXME: this can fail with EOF due to unknown reasons
		log.Error("Error during read")
		return err
	}

	res := new(Rss)
	err = xml.Unmarshal(body, &res)
	if err != nil {
		log.Errorf("Error during Unmarshal")
		return err
	}

	maxCount := show.CurrentCount

	// Use found items in XML
	for _, item := range res.Items {

		if !strings.Contains(item.Title, search_title) {
			continue
		}

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
			} else {
				log.Infof("Pushed '%v' to transmission", item.Title)
				// only update max count if it was successfully pushed
				if count > maxCount {
					maxCount = count
				}
			}
		}
	}

	if maxCount > show.CurrentCount {
		// push update to API
		log.Infof("Update episode of %v to %v", show.Title, maxCount)
		api := Init(config.Api)
		return api.PostEpisodeCount(show.ID, maxCount)
	}

	return nil
}
