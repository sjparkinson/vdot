#!/bin/bash

set -eo pipefail

cargo login "$CARGO_TOKEN"

cargo publish --no-verify --allow-dirty