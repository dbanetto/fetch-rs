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
var clientID = "dd031b32d2f56c990b1425efe6c42ad847e7fe3ab46bf1299f05ecd856bdb7dd"
var clientSecret = "54d7307928f63414defd96399fc31ba847961ceaecef3a5fd93144e960c0e151"

var kitsuCompleted = "completed"
var kitsuCurrent = "current"
var kitsuPlanned = "planned"

// SyncKitsu synchronises episode count with Kitsu
func SyncKitsu(logger *log.Entry, session *KitsuSession, count *fetchapi.InfoBlob, kitsu *fetchapi.InfoBlob) error {

	current := int(count.Blob["current"].(float64))
	total := int(count.Blob["total"].(float64))
	id := fmt.Sprintf("%v", int(kitsu.Blob["id"].(float64)))
	offset := int(kitsu.Blob["offset"].(float64))
	status := kitsuCurrent

	current = current - offset

	if current <= 0 {
		status = kitsuPlanned
	} else if total != 0 && current >= total {
		status = kitsuCompleted
	}

	current = current - offset

	logger = logger.
		WithField("current", current).
		WithField("show_status", status).
		WithField("kitsu_id", id)

	// check if the entry exists
	libraryID := getLibraryEntryID(logger, id, session)

	if libraryID == "" {
		logger.Info("Creating new entry")

		body := generatePOSTBody(current, status, id, session.UserId)

		_, err := kitsuPost(logger, "/api/edge/library-entries", "POST", body, session)

		return err
	}

	// Update
	logger = logger.WithField("entry_id", libraryID)
	logger.Info("Updating entry")
	body := generatePUTBody(current, status, libraryID)

	_, err := kitsuPost(logger, fmt.Sprintf("/api/edge/library-entries/%v", libraryID), "PUT", body, session)

	return err
}

func getLibraryEntryID(logger *log.Entry, showID string, session *KitsuSession) string {
	url := fmt.Sprintf(
		"/api/edge/library-entries?filter[userId]=%v&filter[animeId]=%v&page[limit]=1",
		session.UserId,
		showID)

	bytes, err := kitsuGet(logger, url, session)

	if err != nil {
		log.Error(err)
		return ""
	}

	var entry struct {
		Data []struct {
			ID string `json:"id"`
		} `json:"data"`
	}

	err = json.Unmarshal(bytes, &entry)
	if err != nil {
		log.Error(err)
		return ""
	}

	if len(entry.Data) > 0 {
		return entry.Data[0].ID
	}

	return ""
}

func generatePUTBody(current int, status string, libraryID string) string {
	attr := make(map[string]interface{})
	attr["progress"] = current
	attr["status"] = status

	data := make(map[string]interface{})
	data["type"] = "libraryEntries"
	data["attributes"] = attr

	top := make(map[string]interface{})

	top["data"] = data
	data["id"] = libraryID

	body, err := json.Marshal(top)
	if err != nil {
		log.WithField("body", top).Error(err)
	}

	return fmt.Sprintf("%s", body)
}

func generatePOSTBody(current int, status string, showID string, userID string) string {
	attr := make(map[string]interface{})
	attr["progress"] = current
	attr["status"] = status

	data := make(map[string]interface{})
	data["type"] = "libraryEntries"
	data["attributes"] = attr

	top := make(map[string]interface{})

	top["data"] = data
	userData := make(map[string]interface{})
	userData["type"] = "users"
	userData["id"] = userID // not sure what ID this should be
	user := make(map[string]interface{})
	user["data"] = userData

	mediaData := make(map[string]interface{})
	mediaData["type"] = "anime"
	mediaData["id"] = showID

	media := make(map[string]interface{})
	media["data"] = mediaData

	relationships := make(map[string]interface{})
	relationships["user"] = user
	relationships["media"] = media

	data["relationships"] = relationships

	body, err := json.Marshal(top)
	if err != nil {
		log.WithField("body", top).Error(err)
	}

	return fmt.Sprintf("%s", body)
}

// GetKitsuToken Authenticates against the Kitsu API
func GetKitsuToken(creds KitsuConfig) (KitsuSession, error) {

	session := KitsuSession{}

	form := url.Values{}
	form.Add("grant_type", "password")
	form.Add("username", creds.Username)
	form.Add("password", creds.Password)
	form.Add("client_id", clientID)
	form.Add("client_secret", clientSecret)

	body := form.Encode()

	bytes, err := kitsuPost(log.WithField("kitsu_get_token", true), "/api/oauth/token", "POST", body, nil)

	if err != nil {
		return session, err
	}

	err = json.Unmarshal(bytes, &session)
	if err == nil {
		session.UserId = creds.UserID
	}

	return session, err
}

func kitsuGet(logger *log.Entry, endpoint string, session *KitsuSession) ([]byte, error) {

	client := &http.Client{}

	uri := fmt.Sprint(kitsuAPI, endpoint)

	req, err := http.NewRequest("GET", uri, nil)
	if err != nil {
		return nil, err
	}

	if session != nil {
		req.Header.Add("Authorization", fmt.Sprintf("Bearer %s", session.AccessToken))
	}

	req.Header.Add("Accept", "application/vnd.api+json")
	req.Header.Add("Content-Type", "application/vnd.api+json")

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
		logger.
			WithField("status", resp.StatusCode).
			WithField("uri", uri).
			Info()

		return bytes, statusErr
	}

	logger.
		WithField("status", resp.StatusCode).
		WithField("uri", uri).
		Errorf("%s", err)

	return nil, err
}

func kitsuPost(logger *log.Entry, endpoint string, method string, body string, session *KitsuSession) ([]byte, error) {

	client := &http.Client{}

	uri := fmt.Sprint(kitsuAPI, endpoint)

	req, err := http.NewRequest(method, uri, bytes.NewBufferString(body))
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
		logger.
			WithField("status", resp.StatusCode).
			WithField("uri", uri).
			WithField("method", method).
			Info()

		return bytes, statusErr
	}

	logger.
		WithField("status", resp.StatusCode).
		WithField("uri", uri).
		WithField("method", method).
		Error()

	return nil, err
}

// KitsuSession is an authenicated blob required to make requests
type KitsuSession struct {
	UserId       string `json:"-,omitempty"`
	AccessToken  string `json:"access_token"`
	CreatedAt    int    `json:"created_at"`
	ExpiresIn    int    `json:"expires_in"`
	RefreshToken string `json:"refresh_token"`
	Scope        string `json:"scope"`
	TokenType    string `json:"token_type"`
}
