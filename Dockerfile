FROM rust:alpine3.24 AS builder
WORKDIR /usr/src/url-shortener
COPY . .
RUN cargo install --path .

FROM alpine:latest
RUN apk add --no-cache ca-certificates
WORKDIR /usr/src/url-shortener
RUN mkdir -p data
COPY --from=builder /usr/local/cargo/bin/URL-Shortener /usr/local/bin/URL-Shortener
COPY ./static ./static
EXPOSE 8080
CMD ["URL-Shortener"]
