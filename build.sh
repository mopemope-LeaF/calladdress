set -eu

pushd flipper && cargo +nightly contract build --generate code-only && popd &&
pushd tikitaka && cargo +nightly contract build --generate code-only && popd &&
cargo +nightly contract build
