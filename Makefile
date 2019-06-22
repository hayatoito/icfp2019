TIMESTAMP =  $(shell date '+%Y-%m-%d-%H%M%S')

all: build

run-1:
	cargo run --release -- run --id 1

run-all:
	cargo run --release -- run-all

build:
	cargo build --release

clippy:
	cargo clippy

bench:
	cargo bench

test:
	cargo test --release

ci: test
	cargo run --release -- -v ci

gdb:
	cargo build
	rust-gdb --args ~/src/build/rust/icfp2018/debug/icfp2018 -v run --target ./contest/model/FA001_tgt.mdl

doc:
	cargo doc --open

submit:
	cp -a ./contest/lastrun/*.sol ./contest/submit
	cd ./contest/submit && \
		zip -r ./solutions-$(TIMESTAMP).zip *.sol && \
		cp solutions-$(TIMESTAMP).zip solutions.zip
	cd ./contest/submit && curl -F "private_id=3ca234b530426ee29dcbf7fc" -F "file=@solutions.zip" https://monadic-lab.org/submit

update-best:
	cargo run --release -- -v update-best

submit-best:
	cd ./contest/best && \
		zip -r ./solutions-$(TIMESTAMP).zip *.sol && \
		cp solutions-$(TIMESTAMP).zip solutions.zip
	cd ./contest/best && curl -F "private_id=3ca234b530426ee29dcbf7fc" -F "file=@solutions.zip" https://monadic-lab.org/submit

setup-pipenv:
	cd ./contest/lambda-client && PIPENV_IGNORE_VIRTUALENVS=1 pipenv --python 3.7
	cd ./contest/lambda-client && pipenv shell
	: # $> pipenv install

# zip-trace:
# 	mkdir ./contest/submit/$(TIMESTAMP)
# 	cp -a ./contest/trace/default/*.nbt ./contest/submit/$(TIMESTAMP)
# 	cp -a ./contest/submit/*.nbt ./contest/submit/$(TIMESTAMP)
# 	cd ./contest/submit/$(TIMESTAMP) && zip -r ../$(TIMESTAMP)-trace.zip *.nbt
# 	shasum -a 256 ./contest/submit/$(TIMESTAMP)-trace.zip > ./contest/submit/$(TIMESTAMP)-sha256.txt
# 	cp ./contest/submit/$(TIMESTAMP)-trace.zip ~/drive/public/2018/
# 	cp ./contest/submit/$(TIMESTAMP)-sha256.txt  ~/drive/public/2018/

.PHONY: ci zip
