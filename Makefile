all: debug

debug: 
	cargo build

release:
	cargo build --release

update:
	cargo update

package:
	python package.py

clean:
	rm -rf target
