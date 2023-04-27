FROM golang:1.20.3-bullseye

WORKDIR /app

COPY . /app

RUN go build -o main .

CMD ["./main"]
