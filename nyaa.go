package fetcher

import (
	"encoding/xml"
	"fmt"
	log "github.com/sirupsen/logrus"
	"io/ioutil"
	"net/http"
	"net/url"
	"regexp"
	"strconv"
	"strings"
)

func NyaaFetch(show Series, config Config) error {

	// type def for later
	type Nyaa struct {
		Items []struct {
			Title string `xml:"title"`
			Link  string `xml:"link"`
		} `xml:"channel>item"`
	}

	api := Init(config.Api)
	blobs, err := api.GetInfoBlob(show.ID, []string{"count", "nyaa"})
	if err != nil {
		return err
	}

	nyaaBlob, err := blobs.GetType("nyaa")
	if err != nil {
		return err
	}

	countBlob, err := blobs.GetType("count")
	if err != nil {
		return err
	}

	t, err := buildTransmission(config)
	if err != nil {
		return err
	}

	searchTitle := resolveSearchTitle(show, nyaaBlob)

	// Supported  media type option
	query := url.PathEscape(nyaaBlob.Blob["query"].(string))

	userID := url.PathEscape(nyaaBlob.Blob["user_id"].(string))

	// build the url
	searchURL := fmt.Sprintf("https://www.nyaa.si/?page=rss&user=%v&term=%v+%v",
		userID,
		query,
		url.QueryEscape(searchTitle))

	// logs the resulting URL
	log.WithField("url", searchURL).Info("Searching for ", searchTitle)

	// Build the request
	client := &http.Client{}
	req, err := http.NewRequest("GET", searchURL, nil)
	if err != nil {
		log.WithField("err", err).Errorf("Error during request building")
		return err
	}
	req.Close = true

	// Handle the response
	resp, err := client.Do(req)
	if err != nil {
		log.WithField("err", err).Errorf("Error during request")
		return err
	}
	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil { // FIXME: this can fail with EOF due to unknown reasons
		log.WithField("err", err).Errorf("Error during read")
		return err
	}

	// Parse XML into  struct
	res := new(Nyaa)
	err = xml.Unmarshal(body, &res)
	if err != nil {
		log.WithField("err", err).Errorf("Error during Unmarshal")
		return err
	}

	current := int(countBlob.Blob["current"].(float64))
	newCurrent := current

	// Use found items in XML
	for _, item := range res.Items {
		// strip out title to ensure no missmatches
		epi := strings.Replace(item.Title, searchTitle, "", 1)

		// find count using given regexp
		findCount := regexp.MustCompile("\\d+")
		countMatch := strings.TrimLeft(findCount.FindString(epi), "0")

		count, err := strconv.Atoi(countMatch)
		if err != nil {
			log.Errorf("ERROR parsing episode count (%v): %v", countMatch, err)
		}

		// check if this is a new episode found
		if count > current {
			log.Infof("Found episode %v of %v", count, show.Title)

			// push episode to transmission
			_, err := t.Add(item.Link)
			if err != nil {
				log.Errorf("ERROR while pushing url to transmission (%v): %v", item.Link, err)
			} else {
				log.Infof("Pushed '%v' to transmission", item.Title)
				// only update max count if it was successfully pushed
				if count > newCurrent {
					newCurrent = count
				}
			}
		}
	}

	if newCurrent > current {
		// push update to API
		log.WithField("old_count", current).WithField("new_count", newCurrent).Infof("Update episode of %v to %v", show.Title, newCurrent)
		countBlob.Blob["currnet"] = newCurrent
		return api.PutEpisodeCount(show.ID, *countBlob)
	}

	// everything must of been ffiinnee
	return nil
}
