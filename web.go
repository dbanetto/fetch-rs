package sitesync

import (
	"encoding/json"
	"fmt"
	log "github.com/sirupsen/logrus"
	fetchapi "gitlab.com/zyphrus/fetch-api-go"
	"net/http"
	"strings"
	"time"
)

// StartWeb initialises web API
func StartWeb(config Config) {

	handleFunc("/healthcheck", "GET", config, handleHealthCheck)
	handleFunc("/sync", "POST", config, handleSync)

	addr := config.Web.Host

	log.WithField("host", addr).Info("Starting web interface")

	err := http.ListenAndServe(addr, nil)
	if err != nil {
		log.Fatal("Error during serving web interface", err)
	}
}

func handleHealthCheck(w *loggedRes, r *http.Request, config Config) {

	var res = make(map[string]interface{})
	// check if API is assessible
	api := fetchapi.Init(config.FetchAPI.URL)
	apiStatus := true
	err := api.GetStatus()
	if err != nil {
		apiStatus = false
		log.
			WithField("api", config.FetchAPI.URL).
			WithField("err", err).
			Errorf("Fetch failed healthcheck")
		res["api_error"] = fmt.Sprint(err)
	}

	// check if MAL is assessible
	err = CheckMALCreds(&config.Mal)
	malStatus := true
	if err != nil {
		malStatus = false
		log.WithField("err", err).Error("MAL failed healthcheck")
	}

	// check Kitsu
	_, err = GetKitsuToken(config.Kitsu)
	kitsuStatus := true
	if err != nil {
		kitsuStatus = false
		log.WithField("err", err).Error("Kitsu failed healthcheck")
	}

	res["fetch"] = apiStatus
	res["mal"] = malStatus
	res["kitsu"] = kitsuStatus

	status := 500
	if apiStatus && malStatus && kitsuStatus {
		status = 200
	}

	sendJSON(res, w, status)
}

func handleSync(w *loggedRes, r *http.Request, config Config) {

	var res = make(map[string]interface{})

	err := Run(config)
	if err != nil {
		res["success"] = false
		res["error"] = err
		sendJSON(res, w, 500)
		return
	}

	res["success"] = true
	sendJSON(res, w, 200)
}

func sendJSON(d interface{}, w *loggedRes, status int) {
	bytes, err := json.Marshal(d)

	if err == nil {
		w.Header().Add("Content-Type", "application/json")
		w.SetStatus(status)
		w.Write(bytes)
	} else {
		fmt.Fprint(w, err)
	}
}

func handleFunc(pattern string, method string, config Config, handler func(*loggedRes, *http.Request, Config)) {

	// duplicates the handler with a trailing slash
	if pattern == strings.TrimRight(pattern, "/") {
		handleFunc(pattern+"/", method, config, handler)
	}

	http.HandleFunc(pattern, func(w http.ResponseWriter, r *http.Request) {
		log.Debug("Trying Request for ", r.URL.Path, " (", r.Method, ") against ", pattern)
		matched := pattern == r.URL.Path
		matchedTrim := pattern == strings.TrimRight(r.URL.Path, "/")
		if method == r.Method && (matched || matchedTrim) {
			start := time.Now()
			logged := loggedRes{w, 200}
			handler(&logged, r, config)

			end := time.Now()
			log.
				WithField("path", r.URL.Path).
				WithField("method", r.Method).
				WithField("status", logged.Status).
				WithField("time_delta", end.Sub(start)).
				Info()
		} else {
			http.NotFound(w, r)
		}
	})
}

type loggedRes struct {
	http.ResponseWriter
	Status int
}

func (logged *loggedRes) SetStatus(status int) {
	logged.Status = status
	logged.WriteHeader(status)
}
