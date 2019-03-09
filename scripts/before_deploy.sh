#!/bin/bash

set -eo pipefail

perl -i -pe "s/\"0.0.0\"/\"${TRAVIS_TAG#"v"}\"/" Cargo.toml

cross build --target "$TARGET" --release

tar -C "target/$TARGET/release/" -czf "target/vdot-$TRAVIS_TAG-$TARGET.tar.gz" vdot
