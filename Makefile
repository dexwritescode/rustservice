
.PHONY: builddocker
builddocker:
	docker build -t rustservice .

.PHONY: rundocker
rundocker:
	#docker run -it --rm --name rustservice rustservice
	docker run --rm --name rustservice rustservice

.PHONY: build
build:
	cargo build

.PHONY: run
run:
	cargo run

.PHONY: buildrelease
buildrelease:
	cargo build --release

.PHONY: runrelease
runrelease:
	cargo run --release
