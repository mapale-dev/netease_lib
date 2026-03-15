# netease_lib

Rust library for calling a subset of Netease Cloud Music's **EAPI**.

It provides:

- `NeteaseApiClient` for EAPI request encryption + headers
- typed request builders (e.g. `SearchQuery`, `PlayerUrlRequest`)
- typed response models under `netease_lib::model`

## Quick start

```rust
use netease_lib::{NeteaseApiClient, SearchQuery};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = NeteaseApiClient::new()?;

    let query = SearchQuery::new("周杰伦").with_limit(5);
    let artists = netease_lib::api::search::artist(&client, &query).await?;

    println!("code={}", artists.code);
    Ok(())
}
```

## Notes

- This crate uses async `reqwest`; you need a Tokio runtime in your application.
- API fields may change; models keep an `extra` map to preserve unknown JSON fields.
