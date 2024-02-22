#!/usr/bin/env bash
set -e -o pipefail

TARGET="$1"

echo $TARGET

cargo scout-audit -m $TARGET
