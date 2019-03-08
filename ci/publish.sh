#!/bin/sh

source ~/.cargo/env || true

cargo login $CARGO_TOKEN

cargo publish --allow-dirty