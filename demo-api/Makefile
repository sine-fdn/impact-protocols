.PHONY: all
all:
	@echo "Usage: make <ci|run>"
	@echo "  - ci :: Performs tests"
	@echo "  - run :: Starts the demo API at port 8000"

.PHONY: run
run: keypair.pem
	PRIV_KEY=`cat keypair.pem` cargo run



.PHONY: ci
ci: keypair.pem
	cargo build
	PRIV_KEY=`cat keypair.pem` cargo test

keypair.pem: scripts/keygen.sh
	$<

.PHONY: build-lambda target_lambda/deploy.zip aws-upload
