build:
	@ cargo build -r \
	&& gcc main.c -L ./target/release/ -l cabi -o main

nuke:
	@ cargo clean \
	&& rm main

.PHONY: build nuke
