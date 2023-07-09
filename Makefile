
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

.PHONY: platform-start
platform-start:
	docker-compose -f docker-compose.yml up -d

.PHONY: platform-stop
platform-stop:
	docker-compose -f docker-compose.yml down
