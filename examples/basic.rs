use netease_lib::{NeteaseApiClient, SearchQuery};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = NeteaseApiClient::new()?;

    let banners = netease_lib::api::banner::get(&client, None).await?;
    println!("banners: {}", banners.banners.len());

    let query = SearchQuery::new("周杰伦").with_limit(3);
    let artists = netease_lib::api::search::artist(&client, &query).await?;
    println!("artists code: {}", artists.code);

    Ok(())
}
