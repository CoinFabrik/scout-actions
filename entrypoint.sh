#!/usr/bin/env bash
set -e -o pipefail

TARGET="$1"

echo $TARGET

cd $TARGET
cargo scout-audit
