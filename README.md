## Rust service project template

This template can be used as a starting point for new Rust-based services. It contains a general logger configuration
and a build script that provides build information for the service.

### Usage

This template is supposed to be used with [`cargo-generate`](https://cargo-generate.github.io/cargo-generate/). After
installing `cargo-generate`, simply create a new project with `cargo generate --git git@dev.ukatemi.com/research-and-development/rust-service-template.git`,
and follow the instructions.

If `cargo-generate` is not available for some reason, you can simply clone/download this repository. Don't forget to  
a) change the project name and author in `Cargo.toml`  
b) change the git repo's origin to the project's target remote.

### Help
If you have any questions or problems with this template, ask @mate.magyar.