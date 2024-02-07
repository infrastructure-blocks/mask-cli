# rust-binary-template
[![Build](https://github.com/infrastructure-blocks/rust-binary-template/actions/workflows/build.yml/badge.svg)](https://github.com/infrastructure-blocks/rust-binary-template/actions/workflows/build.yml)
[![Trigger Update From Template](https://github.com/infrastructure-blocks/rust-binary-template/actions/workflows/trigger-update-from-template.yml/badge.svg)](https://github.com/infrastructure-blocks/rust-binary-template/actions/workflows/trigger-update-from-template.yml)

Template repository for rust binaries. Upon instantiating, go through the following checklist:
- Update [package name](./Cargo.toml)
- `cargo build`
- Update the status badges:
    - Remove the `Trigger Update From Template` status badge.
    - Add the `Update From Template` status badge.
    - Rename the rest of the links to point to the right repository.
- Replace/rename the relevant sections in this README
