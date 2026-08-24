#!/usr/bin/env bash

# Boot the build on CHERI QEMU's virt machine with a Morello CPU. Plain virt
# enters the ELF at EL1, where the runtime's hybrid entry enables capabilities
# and switches to C64 on its own. Leave with ctrl+a x.

: "${CRABILITY_BIN:={{crability_bin}}}"

ELF="${1:-target/aarch64-unknown-none-purecap/release/{{project-name}}}"

exec "$CRABILITY_BIN/qemu-system-morello" \
    -M virt \
    -cpu morello \
    -nographic \
    -kernel "$ELF"
