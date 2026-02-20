# Variables
BINARY_NAME=lc3-vm
CARGO=cargo

.PHONY: all build run test fmt clippy clean help

all: fmt clippy test build

build:
	$(CARGO) build --release

run:
	$(CARGO) run

test:
	$(CARGO) test

fmt:
	$(CARGO) fmt --all -- --check

clippy:
	$(CARGO) clippy -- -D warnings

clean:
	$(CARGO) clean

help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@sed -n 's/^##//p' $(MAKEFILE_LIST) | column -t -s ':' |  sed -e 's/^/ /'
