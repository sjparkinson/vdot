#!/bin/bash

cargo login "$CARGO_TOKEN"

cargo publish --no-verify --allow-dirty