fn get_local_mac() -> anyhow::Result<String>{
    let mac = mac_address::get_mac_address()?;
    match mac {
        Some(mac) => Ok(mac.to_string()),
        None => Ok("".to_string())
    }
}

#[test]
fn test_get_local_mac(){
    let mac = get_local_mac().unwrap();
    println!("{}", mac);
}