#!/usr/bin/env bash

: "${CRABILITY_BIN:={{crability_bin}}}"

exec "$CRABILITY_BIN/rustc" "$@"
