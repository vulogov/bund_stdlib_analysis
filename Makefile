SOURCES=$(wildcard src/*.rs)
TESTS=$(wildcard tests/*.rs)


all: $(SOURCES) Makefile
	cargo build

rebuild:
	make clean
	make all

test: $(TESTS)
	cargo test -- --show-output

clean:
	cargo clean
	cargo update

commit:
	aic -ac
	git push
