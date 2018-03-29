package sitesync

import (
	log "github.com/sirupsen/logrus"
	fetchapi "gitlab.com/zyphrus/fetch-api-go"
)

// SyncMAL updates MAL with the current status of the series
func SyncMAL(logger *log.Entry, creds SiteConfig, count *fetchapi.InfoBlob, mal *fetchapi.InfoBlob) error {

	return nil
}
