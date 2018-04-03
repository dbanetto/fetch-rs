package fetcher

import (
	"encoding/json"
	"fmt"
	log "github.com/sirupsen/logrus"
	fetchapi "gitlab.com/zyphrus/fetch-api-go"
	"net/http"
	"os/exec"
	"strings"
	"time"
)

// StartWeb initilising the web API
// Is a blocking call until the server terminates (never)
func StartWeb(config Config) {
	handleFunc("/api/v1/log", "GET", config, handleLog)

	handleFunc("/api/v1/force/fetch", "POST", config, handleForceFetch)

	handleFunc("/api/v1/healthcheck", "GET", config, handleHealthCheck)

	addr := config.WebUI.Host

	log.Info("Server starting to listen on ", addr)

	err := http.ListenAndServe(addr, nil)
	if err != nil {
		log.Fatal("Error during serving web interface", err)
	}
}

func handleLog(w *loggedRes, r *http.Request, config Config) {
	var res = make(map[string]interface{})

	status := 200
	res["success"] = true
	res["log"] = make([]string, 0)
	// HACK: this is a pretty dirty way to read the log
	out, err := exec.Command("journalctl", "--no-pager", "-u", "fetcherd", "--output=cat", "-n", "100").Output()
	if err != nil {
		log.WithField("err", err).Error("Failed to run journalctl command")
		status = 500
	} else {
		res["log"] = strings.Split(string(out), "\n")
	}

	sendJSON(res, w, status)
}

func handleForceFetch(w *loggedRes, r *http.Request, config Config) {
	var res = make(map[string]interface{})

	err := Fetch(config)
	status := 200
	res["success"] = err == nil
	if err != nil {
		res["error"] = fmt.Sprint(err)
		status = 500
	}

	sendJSON(res, w, status)
}

func handleHealthCheck(w *loggedRes, r *http.Request, config Config) {
	var res = make(map[string]interface{})

	// check if API is assessible
	api := fetchapi.Init(config.FetchApi)
	apiStatus := true
	err := api.GetStatus()
	if err != nil {
		apiStatus = false
		log.WithField("api", config.FetchApi).Errorf("Error getting API status: %v", err)
		res["api_error"] = fmt.Sprint(err)
	}

	transmissionStatus := true
	transmission, err := buildTransmission(config)
	if err != nil {
		transmissionStatus = false
		log.WithField("transmission", config.Transmission.Rpc).Errorf("Error creating connection to Transmission: %v", err)
		res["transmission_error"] = fmt.Sprint(err)
	} else {
		_, err := transmission.GetTorrents()
		if err != nil {
			transmissionStatus = false
			log.WithField("transmission", config.Transmission.Rpc).Errorf("Error testing connection to Transmission: %v", err)
			res["transmission_error"] = fmt.Sprint(err)
		}
	}

	res["api"] = apiStatus
	res["transmission"] = transmissionStatus

	status := 500
	if apiStatus && transmissionStatus {
		status = 200
	}

	sendJSON(res, w, status)
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
