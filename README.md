# Hippo

**Hippo** is an experimental compile-time asset preprocessor; its intended usage is to
preprocess web assets with existing toolchains.

> :warning: The scope of the project and its programming interfaces are very likely to
> change.

## Usage

A `struct` or `enum` may derive from `Preprocess` to embed the output of a command. The
processed output can be accessed via `Self::HIPPO_DATA`, `Self::preprocessed_data()` or
its `Display` implementation.

### Rust
```rust
use hippo::Preprocess;

#[derive(Preprocess)]
#[hippo("sass", "css/main.scss")]
pub struct MainCSS;
```

### Hippo.toml
```toml
[sass]
command = "sassc"
flags   = ["-t", "compressed"]
prefix  = "static/"
format  = "utf-8"
```

## Configuration

Hippo will look for a `Hippo.conf` in the crate root; this file defines one or more
preprocessors that will be made available to any `struct` or `enum` that derives from
`Preprocess`.

A preprocessor is identified by its TOML section and its behavior is controlled by its
key-value pairs. The following key-value pairs are recognized:

| Key     | Type         | Description                                                   |
| ------- | ------------ | ------------------------------------------------------------- |
| command | String       | The command to be executed. Mandatory.                        |
| flags   | String Array | A collection of command options. Optional.                    |
| prefix  | String       | A path prefix to be prepended to input arguments. Optional.   |
| format  | String       | The output format. Must be `"bytes"` (default) or `"utf-8"`.  |
