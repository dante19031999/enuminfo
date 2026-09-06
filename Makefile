install:
	cargo install cargo-deny cargo-audit cargo-about --locked

audit:
	cargo audit

deny:
	cargo deny check

doc:
	cargo doc

ackowledgments:
	cargo about generate about.hbs > ACKNOWLEDGEMENTS.html

test:
	cargo test
