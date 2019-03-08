#!/bin/bash

# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=
    local stage=

    src="$(pwd)"

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --bin vdot --target "$TARGET" --release -- -C lto

    cp "target/$TARGET/release/vdot" "$stage/"

    cd "$stage"
    tar czf "$src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz" ./*
    cd "$src"

    rm -rf "$stage"
}

main
