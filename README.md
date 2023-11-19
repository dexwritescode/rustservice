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

<img width="1286" alt="jaeger" src="https://github.com/dexwritescode/todoservice/assets/27153243/283daa79-67e1-44bb-ab94-802b8aee7cb8">

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

### Get Todo
#### Request
```shell
curl --location 'http://localhost:8080/todo/1'
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

### Delete Todo
#### Request
```shell
curl --location --request DELETE 'http://localhost:8080/todo/1'
```
#### Response
200 OK - (Empty response body)

### Get All Todos
#### Request
```shell
curl --location 'http://localhost:8080/todo'
```
#### Response
200 OK
```json
[
    {
        "id": 1,
        "title": "title text",
        "body": "body text",
        "completed": false
    },
    {
        "id": 2,
        "title": "a todo",
        "body": "a todo",
        "completed": true
    },
    {
        "id": 3,
        "title": "Clean out your car",
        "body": "busywork",
        "completed": true
    }
]
```

### Create Random Todo
#### Request
```shell
curl --location --request POST 'http://localhost:8080/todo/random'
```
#### Response
200 OK
```json
{
    "id": 3,
    "title": "Clean out your car",
    "body": "busywork",
    "completed": false
}
```

### Mark Todo Completed
#### Request
```shell
curl --location --request PUT 'http://localhost:8080/todo/3'
```
#### Response
200 OK
```json
{
    "id": 3,
    "title": "Clean out your car",
    "body": "busywork",
    "completed": true
}
```

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

<br>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this repo by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.