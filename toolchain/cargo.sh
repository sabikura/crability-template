#!/usr/bin/env bash

: "${CRABILITY_BIN:={{crability_bin}}}"

# The fork's cargo is only present if x.py built tools/cargo; the host cargo
# works too since .cargo/config.toml points rustc at the Morello compiler.
if [ -x "$CRABILITY_BIN/cargo" ]; then
    exec "$CRABILITY_BIN/cargo" "$@"
fi

exec cargo "$@"
