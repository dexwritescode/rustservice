# todoservice

[![CI](https://github.com/dexwritescode/todoservice/actions/workflows/ci.yaml/badge.svg)](https://github.com/dexwritescode/todoservice/actions/workflows/ci.yaml)

RESTful Rust Todo service

## Running
Start Postgres DB
```console
docker-compose -f docker-compose.yml --profile infra up -d
```

Run the SQL migration
```console
diesel migration run
```

Run todoservice
```console
cargo run
```

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

<br>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this repo by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.