```
cargo clippy : linter 
cargo clippy -- -D warnings : fails build if linter emits a warning

You can mute a warning using the #[allow(clippy::lint_name)] attribute on the affected code
block or disable the noisy lint altogether for the whole project with a configuration line in clippy.toml
12Yes, clippy is named after the (in)famous paperclip-shaped Microsoft Word assistance.
18
or a project-level #![allow(clippy::lint_name)] directive.


######
FMT
######
cargo fmt 
cargo fmt -- --check : fails build if formatter emits warning
```