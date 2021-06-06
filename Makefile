.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check --release

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --release --all

.PHONY: run
run:
	 cargo run --release -- --dev --tmp

.PHONY: build
build:
	 cargo build --release

.PHONY: clean
clean:
	cd ../soul/nikita/chains/realis_network/ && rm -rf db && cd ../../../vlad/chains/realis_network/ && rm -rf db && cd ../../../../Realis.Network

.PHONY: docker 
docker:
	make build && cd target/release && mv realis ../../docker && cd ../../docker && bash ./run-realis.sh

.PHONY: fmt
fmt:
	cargo fmt -p pallet-staking -p pallet-nft -p realis-game-api
