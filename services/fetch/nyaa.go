package fetcher

import (
	"encoding/xml"
	"fmt"
	log "github.com/sirupsen/logrus"
	fetchapi "gitlab.com/zyphrus/fetch-api-go"
	"io/ioutil"
	"net/http"
	"net/url"
	"regexp"
	"strconv"
	"strings"
)

// NyaaFetch searches for the given show using
// the nyaa api and pushes findings to transmission
func NyaaFetch(show fetchapi.Series, config Config) error {

	// type def for later
	type Nyaa struct {
		Items []struct {
			Title string `xml:"title"`
			Link  string `xml:"link"`
		} `xml:"channel>item"`
	}

	api := fetchapi.Init(config.FetchApi)
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

	logger := log.
		WithField("title", show.Title).
		WithField("id", show.ID)

	// Supported  media type option
	query := url.PathEscape(nyaaBlob.Blob["query"].(string))

	userID := url.PathEscape(nyaaBlob.Blob["user_id"].(string))

	// build the url
	searchURL := fmt.Sprintf("https://www.nyaa.si/?page=rss&user=%v&term=%v+%v",
		userID,
		query,
		url.QueryEscape(searchTitle))

	// logs the resulting URL
	logger.
		WithField("search_title", searchTitle).
		WithField("url", searchURL).
		Info("Searching")

	// Build the request
	client := &http.Client{}
	req, err := http.NewRequest("GET", searchURL, nil)
	if err != nil {
		logger.WithField("err", err).Error("Error during request building")
		return err
	}
	req.Close = true

	// Handle the response
	resp, err := client.Do(req)
	if err != nil {
		logger.WithField("err", err).Error("Error during request")
		return err
	}
	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil { // FIXME: this can fail with EOF due to unknown reasons
		logger.WithField("err", err).Error("Error during read")
		return err
	}

	// Parse XML into  struct
	res := new(Nyaa)
	err = xml.Unmarshal(body, &res)
	if err != nil {
		logger.WithField("err", err).Error("Error during Unmarshal")
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

		has, err := regexp.MatchString(regexp.QuoteMeta(searchTitle), item.Title)
		if err != nil {
			logger.
				WithField("err", err).
				Error("ERROR testing item against search title")
		}

		if !has {
			logger.
				WithField("item_title", item.Title).
				WithField("search_title", searchTitle).
				Warn("Skipped item as it did not match")
			continue
		}

		count, err := strconv.Atoi(countMatch)
		if err != nil {
			logger.
				WithField("err", err).
				WithField("count_match", countMatch).
				Error("Parsing episode count")
		}

		// check if this is a new episode found
		if count > current {
			logger.
				WithField("found", count).
				WithField("current", current).
				Info("Found new episode")

			// push episode to transmission
			_, err := t.Add(item.Link)
			if err != nil {
				logger.
					WithField("link", item.Link).
					WithField("err", err).
					Error("ERROR while pushing url to transmission")
			} else {
				logger.
					WithField("link", item.Link).
					Info("Pushed uploaded to transmission")
				// only update max count if it was successfully pushed
				if count > newCurrent {
					newCurrent = count
				}
			}
		}
	}

	if newCurrent > current {
		// push update to API
		logger.
			WithField("old_count", current).
			WithField("new_count", newCurrent).
			Info("Updating episode count")
		countBlob.Blob["current"] = newCurrent
		return api.PutInfoBlob(show.ID, *countBlob)
	}

	logger.Info("Complete")

	// everything must of been ffiinnee
	return nil
}
