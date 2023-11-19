# todoservice

[![CI](https://github.com/dexwritescode/todoservice/actions/workflows/ci.yaml/badge.svg)](https://github.com/dexwritescode/todoservice/actions/workflows/ci.yaml)

RESTful Rust Todo service

## Running
Start Postgres DB
```shell
docker-compose --profile infra up -d
```

Run the SQL migration
```shell
diesel migration run
```

Run todoservice
```shell
cargo run --release
```

Run the tests
```shell
cargo test
```

Start Jaeger all-in-one docker container
```shell
docker-compose --profile tracing up -d
```

Open Jaeger UI on `http://localhost:16686/`

## REST API
### Create Todo
#### Request
```shell
curl --location 'http://localhost:8080/todo' \
--header 'Content-Type: application/json' \
--data '{
    "title":"title text",
    "body":"body text"
}'
```
#### Response
200 OK
```json
{
    "id": 1,
    "title": "title text",
    "body": "body text",
    "completed": false
}
```


## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

<br>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this repo by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.