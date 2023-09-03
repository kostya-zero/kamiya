all: debug

debug: 
	cargo build

release:
	cargo build --release

update:
	cargo update

clean:
	rm -rf target
