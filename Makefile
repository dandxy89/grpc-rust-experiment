# bash
BASE := $(shell /bin/pwd)
POETRY ?= poetry

# WhoAmI?
CONTACT ?= dan.dixey@gmail.com
LOGGING_LEVEL ?= info

# AWS
REGION ?= us-east-1

test:  ## Run the local tests for the FMM Coal Project
	cargo test --locked

install:  ## Install both Prod and Dev Packages in POETRY
	@$(POETRY) install

update_pre_commit:  ## Run `pre-commit` auto update
	@$(POETRY) run pre-commit autoupdate

run_pre_commit:  ## Run `pre-commit` on the project files
	@$(POETRY) run pre-commit run --all-files

get_planets_list:
	grpc_cli call localhost:50051 solar_system.SolarSystem.GetPlanetsList ""

get_planets:
	grpc_cli call localhost:50051 solar_system.SolarSystem.GetPlanet ""

run_server:
	cargo run --bin server --release

run_client:
	cargo run --bin client --release

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
