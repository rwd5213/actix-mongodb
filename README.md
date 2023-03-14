# Actix Mongo

Mostly stolen from <https://blog.devgenius.io/build-a-rest-api-with-rust-and-mongodb-actix-web-version-a275215c262a>

## Run Mongo

docker run --name some-mongo -d mongo:latest

connection string = mongodb://some-mongo{or docker ip}:27017

## Curl Test

```bash
curl -XPOST -H "Content-Type: application/json"  http://localhost:8080/user -d '{"name": "Ryan", "location": "Pittsburgh", "title": "software"}'
```
