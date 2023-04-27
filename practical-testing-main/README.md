# Practical-Testing

## Node
```bash
npm install
node app.js
```
Suggestion: 
- Base-image: node:19.8-bullseye-slim
- Testing: curl localhost:3000

## Go
```bash
go build main.go
./main
```

Suggestion: 
- Base-image: golang:1.20.3-bullseye
- Testing: curl localhost:8080

## Rust
```bash
rustc main.rs
./main
```
Suggestion: 
- Base-image: rust:1.68.2-slim-bullseye
- Testing: curl localhost:8080
