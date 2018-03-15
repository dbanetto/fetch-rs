package fetcher

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	log "github.com/sirupsen/logrus"
	"io/ioutil"
	"net/http"
	"net/url"
	"path"
	"strings"
)

// Init creates an API with the given configuration
func Init(url string) *API {
	api := new(API)
	api.url = url
	api.client = &http.Client{}

	return api
}

func (api *API) get(endpoint string) ([]byte, error) {
	apiUrl, err := url.Parse(api.url)
	if err != nil {
		return nil, err
	}

	apiUrl.Path = path.Join(apiUrl.Path, endpoint)
	r, err := http.NewRequest("GET", apiUrl.String(), nil)
	if err != nil {
		return nil, err
	}

	r.Header.Add("Content-Type", "application/json")

	log.Debug("sending Fetch API get request to ", endpoint)
	resp, err := api.client.Do(r)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	log.Debug("reading response to request for ", endpoint)
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	return body, nil
}

// GetStatus checks if the API endpoint is accessible
func (api *API) GetStatus() error {
	_, err := api.get("/api/v1")

	return err
}

// GetSeries returns a list of all series from the API
func (api *API) GetSeries() ([]Series, error) {
	body, err := api.get("/api/v1/series/")

	if err != nil {
		return nil, err
	}

	series := new(SeriesResponse)
	err = json.Unmarshal(body, &series)

	if err != nil {
		return nil, err
	}

	if !series.Success {
		return nil, errors.New(series.Error)
	}

	return series.Data, nil
}

// GetInfoBlob returns all info blobs of given types
// assoicated to the series id
func (api *API) GetInfoBlob(seriesID int, types []string) (InfoBlobs, error) {

	endPoint := fmt.Sprintf("/api/v1/info/%v/types/%s", seriesID, strings.Join(types, "+"))
	body, err := api.get(endPoint)

	if err != nil {
		return nil, err
	}

	infoblob := new(InfoBlobRespone)
	err = json.Unmarshal(body, &infoblob)

	if err != nil {
		return nil, err
	}

	if !infoblob.Success {
		return nil, errors.New(infoblob.Error)
	}

	return infoblob.Data, nil
}

func (api *API) PutEpisodeCount(seriesId int, count InfoBlob) error {

	data, err := json.Marshal(count)

	apiUrl, err := url.Parse(api.url)
	if err != nil {
		return err
	}

	apiUrl.Path = path.Join(apiUrl.Path, "api", "v1", "info", fmt.Sprint(seriesId), fmt.Sprint(count.ID))
	r, err := http.NewRequest("PUT", apiUrl.String(), bytes.NewBuffer(data))
	if err != nil {
		return err
	}
	r.Header.Add("Content-Type", "application/json")

	_, err = api.client.Do(r)

	return err
}

// API is a struct to interact with the API
type API struct {
	url    string
	client *http.Client
}

// Series is a single series
type Series struct {
	ID        int    `json:"id"`
	PosterURL string `json:"poster_url"`
	Title     string `json:"title"`
}

// SeriesResponse is the API reponse for all series
type SeriesResponse struct {
	Data    []Series `json:"data",omitempty`
	Error   string   `json:"error",omitempty`
	Success bool     `json:"success"`
}

// InfoBlobRespone is a reponse to any /info/ API call
type InfoBlobRespone struct {
	Data    []InfoBlob `json:"data",omitempty`
	Error   string     `json:"error",omitempty`
	Success bool       `json:"success"`
}

// InfoBlob a single blob from the API
type InfoBlob struct {
	Blob     map[string]interface{} `json:"blob"`
	ID       int                    `json:"id"`
	InfoType string                 `json:"info_type"`
}

type InfoBlobs []InfoBlob
