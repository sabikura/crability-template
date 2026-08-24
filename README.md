# crability-template

Cargo-generate templates for pure-capability bare-metal Rust on Arm Morello. The
repo holds two sub-templates, one per execution platform:

- `qemu/` boots on CHERI QEMU's `virt` machine. QEMU enters the ELF at EL1 in
  A64, directly into `aarch64-purecap-rt`'s `_el1_entry`; the runtime's `hybrid`
  feature enables capability instructions and switches to C64 on its own, so
  there is no boot shim. `cargo run` boots QEMU.
- `fvp/` boots on the Morello FVP as the BL33 payload of an Arm Trusted
  Firmware image. A small EL2 stub (`src/el2/entry.S`) drops into
  pure-capability EL1 before the runtime takes over.

Generate a project by naming the sub-template (the first one is the default):

```shell
cargo generate --path <this repo> qemu
cargo generate --path <this repo> fvp
```

Both templates take the same placeholder: `crability_bin`, the directory where
[crability](https://github.com/sabikura/crability) installs
the toolchain.
