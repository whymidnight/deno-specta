# deno-specta

This library will generate a `runtime.js` for your Deno Runtime and an `abi.ts` to contextualize ts conventions against.

## Convention

### To generate a `runtime.js` from your `app`:

#### Import `deno-specta` via:

```
[build-dependencies]
deno_specta = { git = "https://github.com/whymidnight/deno-specta.git", features = ["runtime"] }
specta = { git = "https://github.com/whymidnight/specta.git" }

[dependencies]
deno_specta = { git = "https://github.com/whymidnight/deno-specta.git", features = ["runtime"] }
specta = { git = "https://github.com/whymidnight/specta.git" }
```

#### Feature guard `specta`/`codegen` and `deno_core::op`/`deno_op`:

```
[features]
default = []
codegen = []
deno_op = []
```

#### Declare a function:

```rust
use specta::specta;
use deno_core::op;

#[cfg_attr(feature = "deno_op", op)]
#[cfg_attr(feature = "codegen", specta)]
pub async fn foo(
    my_name: String,
    your_name: String,
) -> Result<String, deno_core::anyhow::Error> {
    Ok(format!(
        "Hello, {my_name}! You've been greeted by {your_name}!"
    ))
}
```

#### Then in `build.rs`:

```rust
use deno_specta::runtime;

runtime::export(
    collect_types![foo],
    "./runtime.js",
)
.unwrap();
```
