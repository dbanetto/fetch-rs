package sitesync

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io/ioutil"
	"net/http"
	"net/url"

	log "github.com/sirupsen/logrus"
	fetchapi "gitlab.com/zyphrus/fetch-api-go"
)

// var kitsuAPI = "http://localhost:8080"

var kitsuAPI = "https://kitsu.io"
var clientId = "dd031b32d2f56c990b1425efe6c42ad847e7fe3ab46bf1299f05ecd856bdb7dd"
var clientSecret = "54d7307928f63414defd96399fc31ba847961ceaecef3a5fd93144e960c0e151"

var entryJSON = `{ "data": {
	"type: "libraryEntries",
	"attributes": {
		"status": "%v",
		"progress": %v
	},
}}`

// SyncKitsu synchronises episode count with Kitsu
func SyncKitsu(logger *log.Entry, session KitsuSession, count *fetchapi.InfoBlob, mal *fetchapi.InfoBlob) error {

	// check if the entry exists

	// create if required

	// or update

	return nil
}

// GetKitsuToken Authenticates against the Kitsu API
func GetKitsuToken(creds SiteConfig) (KitsuSession, error) {

	session := KitsuSession{}

	form := url.Values{}
	form.Add("grant_type", "password")
	form.Add("username", creds.Username)
	form.Add("password", creds.Password)
	form.Add("client_id", clientId)
	form.Add("client_secret", clientSecret)

	body := form.Encode()

	bytes, err := kitsuPost("/api/oauth/token", body, nil)
	if err != nil {
		return session, err
	}

	err = json.Unmarshal(bytes, &session)

	return session, err
}

func kitsuGet(endpoint string, body string, session *KitsuSession) ([]byte, error) {

}

func kitsuPost(endpoint string, body string, session *KitsuSession) ([]byte, error) {

	client := &http.Client{}

	uri := fmt.Sprint(kitsuAPI, endpoint)

	log.WithField("url", uri).Info("Sending POST")

	req, err := http.NewRequest("POST", uri, bytes.NewBufferString(body))
	if err != nil {
		return nil, err
	}

	if session != nil {
		req.Header.Add("Authorization", fmt.Sprintf("Bearer %s", session.AccessToken))
	}

	req.Header.Add("Accept", "application/vnd.api+json")
	req.Header.Add("Content-Length", fmt.Sprint(len(body)))

	if session != nil {
		req.Header.Add("Content-Type", "application/vnd.api+json")
	} else {
		req.Header.Add("Content-Type", "application/x-www-form-urlencoded")
	}

	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}

	defer resp.Body.Close()

	var statusErr error

	if resp.StatusCode > 400 {
		statusErr = errors.New(resp.Status)
	}

	bytes, err := ioutil.ReadAll(resp.Body)

	if err == nil {
		log.
			WithField("status", resp.StatusCode).
			WithField("uri", uri).
			Infof("Successfull POST")

		return bytes, statusErr
	}

	log.
		WithField("status", resp.StatusCode).
		WithField("uri", uri).
		Error("Unsuccessful POST")

	return nil, err
}

// KitsuSession is an authenicated blob required to make requests
type KitsuSession struct {
	AccessToken  string `json:"access_token"`
	CreatedAt    int    `json:"created_at"`
	ExpiresIn    int    `json:"expires_in"`
	RefreshToken string `json:"refresh_token"`
	Scope        string `json:"scope"`
	TokenType    string `json:"token_type"`
}
