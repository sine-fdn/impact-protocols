help:
	@echo make "<help|rebuild-schemas|build|clean|test|start-demo-api>"


rebuild-schemas:
	@echo "Rebuilding schemas..."
	cargo run --bin ileap-data-model
	cargo run --bin pact-data-model

build:
	cargo build

test:
	cargo test

clean:
	cargo clean

ci:
	@echo "Running CI tasks..."
	cd ileap-data-model && cargo test
	cd pact-data-model && cargo test
	make -C demo-api ci


start-demo-api:
	make -C demo-api run


.PHONY: help rebuild-schemas build test clean
