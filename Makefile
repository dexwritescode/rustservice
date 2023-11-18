

.PHONY: help
help: ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)


.PHONY: builddocker
builddocker: ## Build the docker image
	docker build -t todoservice .

.PHONY: rundocker
rundocker: ## Run the service container
	docker run --rm --name todoservice todoservice

.PHONY: build
build: ## Compile the service
	cargo build

.PHONY: run
run: ## Compile and run the service
	cargo run

.PHONY: buildrelease
buildrelease: ## Compile the service in release mode
	cargo build --release

.PHONY: runrelease
runrelease: ## Compile and run the service in release mode
	cargo run --release

.PHONY: start-infra
start-infra: ## Start the service and its infrastructure dependencies
	docker-compose -f docker-compose.yml --profile infra up -d

.PHONY: stop-infra
stop-infra: ## Stop the service and its infrastructure dependencies
	docker-compose -f docker-compose.yml --profile infra stop

.PHONY: start
start: builddocker ## Start the service
	docker-compose -f docker-compose.yml --profile service up -d

.PHONY: stop
stop: ## Stop the service
	docker-compose -f docker-compose.yml --profile service stop

.PHONY: start-tracing
start-tracing: ## Start Jaeger tracing
	docker-compose -f docker-compose.yml --profile tracing up -d

.PHONY: stop-tracing
stop-tracing: ## Stop Jaeger tracing
	docker-compose -f docker-compose.yml --profile tracing stop