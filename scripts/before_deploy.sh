#!/usr/bin/env bash

set -eo pipefail

# Update the crates version in Cargo.toml from the git tag
perl -i -pe "s/\"0.0.0\"/\"${TRAVIS_TAG#"v"}\"/" Cargo.toml

cargo build --target "$TARGET" --release

# Copy into the release any files to be packaged with the executable
cp README.md LICENSE "target/$TARGET/release/"

# Package up the release into a .tar.gz
tar -C "target/$TARGET/release/" -czf "target/vdot-$TRAVIS_TAG-$TARGET.tar.gz" vdot README.md LICENSE
