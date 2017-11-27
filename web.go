package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"strings"
)

func Start(config WebUIConfig) {
	handleFunc("/", "GET", handleInfo)
	handleFunc("/log", "GET", handleLog)
	handleFunc("/status", "GET", handleStatus)

	handleFunc("/force/fetch", "POST", handleForceFetch)
	handleFunc("/force/sort", "POST", handleForceSort)

	addr := config.Host

	log.Printf("Server starting to listen on %v", addr)
	log.Fatal(http.ListenAndServe(addr, nil))
}

func handleInfo(w http.ResponseWriter, r *http.Request) {
	fmt.Fprint(w, "API is online")
}

func handleStatus(w http.ResponseWriter, r *http.Request) {
	var res = make(map[string]interface{})

	res["running"] = true
	res["fetch_lock"] = true
	res["sort_lock"] = true

	sendJson(res, w)
}

func handleLog(w http.ResponseWriter, r *http.Request) {
	var res = make(map[string]interface{})

	res["success"] = true
	res["log"] = "Placeholder log"

	sendJson(res, w)
}

func handleForceFetch(w http.ResponseWriter, r *http.Request) {
	var res = make(map[string]interface{})

	res["success"] = true

	sendJson(res, w)
}

func handleForceSort(w http.ResponseWriter, r *http.Request) {
	var res = make(map[string]interface{})

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

func handleFunc(pattern string, method string, handler func(http.ResponseWriter, *http.Request)) {

	// patt := strings.TrimRight(pattern, "/")

	http.HandleFunc(pattern, func(w http.ResponseWriter, r *http.Request) {
		log.Printf("Trying Request for %v (%v) against %v", r.URL.Path, r.Method, pattern)
		matched := pattern == r.URL.Path
		matchedTrim := pattern == strings.TrimRight(r.URL.Path, "/")
		if method == r.Method && (matched || matchedTrim) {
			// debug
			log.Printf("Matched for %v", pattern)
			handler(w, r)
		} else {
			http.NotFound(w, r)
		}
	})
}
