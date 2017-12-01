package fetcher

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"strings"
)

func StartWeb(config Config) {
	handleFunc("/", "GET", config, handleInfo)
	handleFunc("/log", "GET", config, handleLog)
	handleFunc("/status", "GET", config, handleStatus)

	handleFunc("/force/fetch", "POST", config, handleForceFetch)
	handleFunc("/force/sort", "POST", config, handleForceSort)

	addr := config.WebUI.Host

	log.Printf("Server starting to listen on %v", addr)
	log.Fatal(http.ListenAndServe(addr, nil))
}

func handleInfo(w http.ResponseWriter, r *http.Request, config Config) {
	fmt.Fprint(w, "API is online")
}

func handleStatus(w http.ResponseWriter, r *http.Request, config Config) {
	var res = make(map[string]interface{})

	res["running"] = true
	res["fetch_lock"] = false
	res["sort_lock"] = false

	sendJson(res, w)
}

func handleLog(w http.ResponseWriter, r *http.Request, config Config) {
	var res = make(map[string]interface{})

	res["success"] = true
	res["log"] = "Placeholder log"

	sendJson(res, w)
}

func handleForceFetch(w http.ResponseWriter, r *http.Request, config Config) {
	var res = make(map[string]interface{})

	Fetch(config)
	res["success"] = true

	sendJson(res, w)
}

func handleForceSort(w http.ResponseWriter, r *http.Request, config Config) {
	var res = make(map[string]interface{})

	// TODO: run sort command
	res["success"] = true

	sendJson(res, w)
}

func sendJson(d interface{}, w http.ResponseWriter) {
	bytes, err := json.Marshal(d)

	if err == nil {
		w.Header().Add("Content-Type", "application/json")
		w.Write(bytes)
	} else {
		fmt.Fprint(w, err)
	}
}

func handleFunc(pattern string, method string, config Config, handler func(http.ResponseWriter, *http.Request, Config)) {

	http.HandleFunc(pattern, func(w http.ResponseWriter, r *http.Request) {
		log.Printf("Trying Request for %v (%v) against %v", r.URL.Path, r.Method, pattern)
		matched := pattern == r.URL.Path
		matchedTrim := pattern == strings.TrimRight(r.URL.Path, "/")
		if method == r.Method && (matched || matchedTrim) {
			// debug
			log.Printf("Matched for %v", pattern)
			handler(w, r, config)
		} else {
			http.NotFound(w, r)
		}
	})
}
