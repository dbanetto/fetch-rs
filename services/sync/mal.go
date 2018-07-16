package sitesync

import (
	"bytes"
	"errors"
	"fmt"
	"net/http"

	log "github.com/sirupsen/logrus"
	fetchapi "gitlab.com/zyphrus/fetch-api-go"
)

var malAPIURL = "https://myanimelist.net/api"

var watchingStatus = 1
var completedStatus = 2
var planningStatus = 6

var xmlTemplate = `<?xml version="1.0" encoding="utf-8"?> <entry><episode>%v</episode><status>%v</status>%v</entry>`

// SyncMAL updates MAL with the current status of the series
func SyncMAL(logger *log.Entry, creds *SiteConfig, count *fetchapi.InfoBlob, mal *fetchapi.InfoBlob) error {

	current := int(count.Blob["current"].(float64))
	total := int(count.Blob["total"].(float64))
	id := fmt.Sprint(mal.Blob["id"])
	offset := int(mal.Blob["offset"].(float64))
	status := watchingStatus

	if current <= 0 {
		status = planningStatus
	} else if total != 0 && current >= total {
		status = completedStatus
	}

	current = current - offset

	logger = logger.
		WithField("current", current).
		WithField("status", status).
		WithField("mal_id", id)

	logger.Info("syncing show to MAL")

	xml := buildXML(status, current)

	// ignore error for add, it happens a lot
	_ = malPost(fmt.Sprintf("/animelist/add/%v.xml", id), xml, creds, logger)
	err := malPost(fmt.Sprintf("/animelist/update/%v.xml", id), xml, creds, logger)

	return err
}

// CheckMALCreds ensures that the given credentials are valid
func CheckMALCreds(creds *SiteConfig) error {
	return malGet("/account/verify_credentials.xml", creds, nil)
}

func buildXML(status int, count int) string {
	tags := ""
	if status == watchingStatus {
		tags = "\n\t<tags>aired</tags>"
	}

	return fmt.Sprintf(xmlTemplate, count, status, tags)
}

func malGet(endpoint string, creds *SiteConfig, logger *log.Entry) error {
	if logger == nil {
		logger = log.WithField("no_logger", true)
	}
	client := &http.Client{}

	uri := fmt.Sprint(malAPIURL, endpoint)
	req, err := http.NewRequest("GET", uri, nil)
	if err != nil {
		return err
	}

	req.SetBasicAuth(creds.Username, creds.Password)

	resp, err := client.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	logger.
		WithField("status", resp.StatusCode).
		WithField("uri", uri).
		Info()

	if resp.StatusCode > 400 {
		return errors.New(resp.Status)
	}

	return nil
}

func malPost(endpoint string, body string, creds *SiteConfig, logger *log.Entry) error {
	if logger == nil {
		logger = log.WithField("no_logger", true)
	}

	client := &http.Client{}

	uri := fmt.Sprint(malAPIURL, endpoint)
	form := fmt.Sprintf("data=%s", body)

	req, err := http.NewRequest("POST", uri, bytes.NewBufferString(form))
	if err != nil {
		return err
	}

	req.SetBasicAuth(creds.Username, creds.Password)
	req.Header.Add("Content-Type", "application/x-www-form-urlencoded")
	req.Header.Add("Content-Length", fmt.Sprint(len(form)))

	resp, err := client.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	logger.
		WithField("status", resp.StatusCode).
		WithField("uri", uri).
		Info()

	if resp.StatusCode > 400 {
		return errors.New(resp.Status)
	}

	return nil
}
