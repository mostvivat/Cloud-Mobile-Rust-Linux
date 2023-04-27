package main

import (
    "fmt"
    "net/http"
)

func main() {
	fmt.Println("----- Go Server Starting -------")
    http.HandleFunc("/", Server)
    http.ListenAndServe(":8080", nil)

}

func Server(w http.ResponseWriter, r *http.Request) {
	fmt.Println(r.Proto, r.Method, r.URL)

    fmt.Fprintf(w, "Hello World, Say Hi! Jumpbox")
}