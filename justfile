check:
    cargo check --workspace --all-targets

check-fmt:
    cargo fmt --all -- --check

clippy:
    cargo clippy --workspace --all-targets --all-features --  -D warnings -W clippy::all

test:
    cargo test --workspace --all-targets --all-features

qa: check check-fmt clippy test

fmt:
    cargo fmt --all

sort:
	cargo sort --grouped .

clippy-fix:
    cargo clippy --fix --workspace --all-targets --all-features --allow-dirty

fix: fmt sort clippy-fix

commit message: fix qa
    git commit -am "{{ message }}"

deploy name="routine": qa
    cargo shuttle deploy --name {{name}} --no-test

watch name="routine":
    cargo watch -x 'shuttle run --name {{name}}' -i 'routine-app,Cargo.lock'
