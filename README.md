# netease_lib

`netease_lib` is a Rust library that calls a subset of Netease Cloud Music **EAPI**.

It includes:

- `NeteaseApiClient`: builds EAPI headers, encrypts request body, parses/decrypts response
- typed request builders (e.g. `SearchQuery`, `PlayerUrlRequest`)
- typed response models under `netease_lib::model` (unknown fields are preserved in `extra`)

This crate is async and uses `reqwest`.

## Add Dependency

### Use as a local path dependency

```toml
[dependencies]
netease_lib = { path = "../netease_lib" }
anyhow = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

### Use from Git

```toml
[dependencies]
netease_lib = { git = "https://github.com/mapale-dev/netease_lib.git" }
anyhow = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust
use netease_lib::{NeteaseApiClient, SearchQuery};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = NeteaseApiClient::new()?;

    let query = SearchQuery::new("周杰伦").with_limit(5);
    let resp = netease_lib::api::search::artist(&client, &query).await?;
    println!("code={} artists={}", resp.code, resp.result.artists.len());

    Ok(())
}
```

## Project Structure

- `src/api/*`: API wrappers (request building + EAPI call)
- `src/model/*`: response structs (serde)
- `src/utils/*`: crypto/sign helpers (EAPI encryption/decryption)
- `examples/basic.rs`: runnable example

## API Usage

Most APIs expose:

- a simple function (e.g. `get(...)`, `chorus(...)`, `url_v1(...)`)
- an advanced `fetch_*` function that accepts a typed request struct

### Banner

```rust
use netease_lib::NeteaseApiClient;

# #[tokio::main]
# async fn main() -> anyhow::Result<()> {
let client = NeteaseApiClient::new()?;
let resp = netease_lib::api::banner::get(&client, None).await?;
println!("{}", resp.banners.len());
# Ok(())
# }
```

Advanced:

```rust
use netease_lib::{BannerRequest, NeteaseApiClient};

# #[tokio::main]
# async fn main() -> anyhow::Result<()> {
let client = NeteaseApiClient::new()?;
let req = BannerRequest::new().with_client_type("pc");
let resp = netease_lib::api::banner::fetch(&client, &req).await?;
println!("{}", resp.code);
# Ok(())
# }
```

### Lyric

```rust
use netease_lib::NeteaseApiClient;

# #[tokio::main]
# async fn main() -> anyhow::Result<()> {
let client = NeteaseApiClient::new()?;
let resp = netease_lib::api::lyric::get(&client, 33894312).await?;
println!("{}", resp.code);
# Ok(())
# }
```

Advanced:

```rust
use netease_lib::{LyricRequest, NeteaseApiClient};

# #[tokio::main]
# async fn main() -> anyhow::Result<()> {
let client = NeteaseApiClient::new()?;
let req = LyricRequest::new(33894312);
let resp = netease_lib::api::lyric::fetch(&client, &req).await?;
println!("{}", resp.code);
# Ok(())
# }
```

### Search

`SearchQuery` is used by multiple endpoints:

```rust
use netease_lib::{NeteaseApiClient, SearchQuery};

# #[tokio::main]
# async fn main() -> anyhow::Result<()> {
let client = NeteaseApiClient::new()?;
let q = SearchQuery::new("周杰伦").with_limit(10).with_offset(0);

let artists = netease_lib::api::search::artist(&client, &q).await?;
let albums = netease_lib::api::search::album(&client, &q).await?;
let playlists = netease_lib::api::search::playlist(&client, &q).await?;

println!("{} {} {}", artists.code, albums.code, playlists.code);
# Ok(())
# }
```

Mlog/MV search:

```rust
use netease_lib::{NeteaseApiClient, SearchQuery};

# #[tokio::main]
# async fn main() -> anyhow::Result<()> {
let client = NeteaseApiClient::new()?;
let q = SearchQuery::new("晴天").with_limit(5);
let resp = netease_lib::api::search::mlog(&client, &q, Some("MV")).await?;
println!("{}", resp.code);
# Ok(())
# }
```

### Song

Chorus:

```rust
use netease_lib::NeteaseApiClient;

# #[tokio::main]
# async fn main() -> anyhow::Result<()> {
let client = NeteaseApiClient::new()?;
let resp = netease_lib::api::song::chorus(&client, &[33894312, 186016]).await?;
println!("{}", resp.code);
# Ok(())
# }
```

Player URL v1:

```rust
use netease_lib::NeteaseApiClient;

# #[tokio::main]
# async fn main() -> anyhow::Result<()> {
let client = NeteaseApiClient::new()?;
let resp = netease_lib::api::song::enhance::player::url_v1(
    &client,
    &[33894312],
    Some("exhigh"),
    Some("aac"),
    Some("c51"),
)
.await?;

println!("{}", resp.code);
# Ok(())
# }
```

## Client Configuration

### Base URL

```rust
use netease_lib::NeteaseApiClient;

let client = NeteaseApiClient::with_base_url("https://interface.music.163.com")?;
```

## How It Works (EAPI)

- Request payload is wrapped with `header` + `e_r=true`, serialized, then encrypted into `params=...`
- Response parsing tries:
  - plain JSON
  - hex ciphertext JSON (AES-ECB)
  - binary ciphertext JSON (AES-ECB)

## Run This Repo

```bash
cargo test
cargo check --examples
cargo run --example basic
```

## Build Artifacts

This crate is configured as `rlib`, `staticlib`, and `cdylib`.

```bash
cargo build --release
```

Outputs are in `target/release/`.

Note: this repo currently does not expose a stable C ABI (no `extern "C"` exports yet). `staticlib/cdylib` is enabled for future integration.
