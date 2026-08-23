#!/usr/bin/env bash

# Boot the build on CHERI QEMU's virt machine with a Morello CPU.
{% if platform == "qemu-el1" -%}
# Plain virt enters the ELF at EL1, where the runtime's hybrid entry enables
# capabilities and switches to C64 on its own.
{% else -%}
# virtualization=on enters the ELF at EL2, where the EL2 stub drops to purecap EL1.
{% endif -%}
# -semihosting lets the guest terminate QEMU with a chosen exit code (SYS_EXIT),
# which is how the binaries and the tests report success or failure.
# Leave with ctrl+a x.

: "${CRABILITY_BIN:={{crability_bin}}}"

ELF="${1:-target/aarch64-unknown-none-purecap/release/{{project-name}}}"

exec "$CRABILITY_BIN/qemu-system-morello" \
    -M virt{% if platform != "qemu-el1" %},virtualization=on{% endif %} \
    -cpu morello \
    -nographic \
    -semihosting \
    -kernel "$ELF"
