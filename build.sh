set -eu

pushd tikitaka && cargo +nightly contract build --generate code-only && popd &&
cargo +nightly contract build
