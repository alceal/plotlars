audit:
	@cargo audit
	@cargo clean

autoinherit:
	@cargo autoinherit
	@cargo clean

cache:
	@cargo cache --autoclean

check:
	@cargo check
	@cargo clean

clippy:
	@cargo clippy
	@cargo clean

clean:
	@cargo clean-all -1

crate +args:
	@cargo crate {{args}}
	@cargo clean

coverage:
	@cargo tarpaulin --out stdout lcov
	@cargo clean

doc:
	@cargo doc
	@cargo clean

fix:
	@cargo fix
	@cargo clean

flamegraph +args:
	@CARGO_PROFILE_RELEASE_DEBUG=true sudo cargo flamegraph {{args}}
	@open -a "Safari" flamegraph.svg
	@sudo rm -rf target # We have to remove the target folder because it's owned by root and it conflicts with other commands

fmt:
	@cargo fmt
	@cargo clean

format:
	@cargo check
	@cargo fmt
	@cargo funnel
	@cargo clean

funnel:
	@cargo funnel
	@cargo clean

install crate:
	@cargo binstall {{crate}} --locked -y
	@cargo clean

license:
	@cargo license
	@cargo clean

machete:
	@cargo machete
	@cargo clean

msrv:
	@cargo msrv
	@cargo clean

outdated:
	@cargo outdated
	@cargo clean

readme:
	@cargo rdme
	@cargo clean

semver-check:
	@cargo semver-check
	@cargo clean

test:
	@cargo nextest run
	@cargo clean

uninstall crate:
	@cargo uninstall {{crate}}
	@cargo clean

update:
	@# cargo update # Verify that it also updates the Cargo.toml file
	@# cargo clean

updater:
	@cargo updater --update
	@cargo cache --autoclean

yank version:
	@cargo yank --version {{version}}
	@cargo clean

all: format check clippy test coverage machete audit
