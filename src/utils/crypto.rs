use aes::cipher::block_padding::Pkcs7;
use aes::Aes128;
use ecb::cipher::{BlockDecryptMut, BlockEncryptMut, KeyInit};
use ecb::{Decryptor, Encryptor};

type Aes128EcbEnc = Encryptor<Aes128>;
type Aes128EcbDec = Decryptor<Aes128>;

pub fn aes_ecb_decrypt(text: &str, key: &str) -> anyhow::Result<String> {
    let bytes = hex::decode(text)?;
    let decrypted = aes_ecb_decrypt_bytes(&bytes, key)?;
    Ok(String::from_utf8(decrypted)?)
}

pub fn aes_ecb_decrypt_bytes(ciphertext: &[u8], key: &str) -> anyhow::Result<Vec<u8>> {
    if key.len() != 16 {
        anyhow::bail!("Key must be 16 bytes for AES-128");
    }
    let mut data = ciphertext.to_vec();
    let decrypted = Aes128EcbDec::new(key.as_bytes().into())
        .decrypt_padded_mut::<Pkcs7>(&mut data)
        .map_err(|e| anyhow::anyhow!("Decrypt error: {:?}", e))?;
    Ok(decrypted.to_vec())
}

pub fn aes_ecb_encrypt(text: &str, key: &str) -> anyhow::Result<String> {
    if key.len() != 16 {
        anyhow::bail!("Key must be 16 bytes for AES-128");
    }

    let mut data = hex::decode(text)?;
    let data_len = data.len();
    data.resize(data_len + 16, 0);

    let encrypted = Aes128EcbEnc::new(key.as_bytes().into())
        .encrypt_padded_mut::<Pkcs7>(&mut data, data_len)
        .map_err(|e| anyhow::anyhow!("Encrypt error: {:?}", e))?;

    Ok(hex::encode(encrypted))
}

pub fn netease_res_decrypt(text: &str) -> anyhow::Result<String> {
    aes_ecb_decrypt(text, "e82ckenh8dichen8")
}

pub fn netease_res_decrypt_bytes(ciphertext: &[u8]) -> anyhow::Result<Vec<u8>> {
    aes_ecb_decrypt_bytes(ciphertext, "e82ckenh8dichen8")
}

pub fn netease_req_encrypt(url: &str, text: &str) -> anyhow::Result<String> {
    let message = format!("nobody{}use{}md5forencrypt", url, text);
    let digest = md5::compute(message);
    let data = format!("{}-36cd479b6b5-{}-36cd479b6b5-{digest:x}", url, text);
    let data = hex::encode(data);

    aes_ecb_encrypt(&data, "e82ckenh8dichen8")
}

#[test]
fn test_aes_ecb_roundtrip() {
    let key = "e82ckenh8dichen8";
    let plaintext = "hello-netease";
    let hex_plaintext = hex::encode(plaintext.as_bytes());
    let encrypted = aes_ecb_encrypt(&hex_plaintext, key).unwrap();
    let decrypted = aes_ecb_decrypt(&encrypted, key).unwrap();
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_netease_req_encrypt() {
    let params = netease_req_encrypt("/api/v2/banner/get", r#"{"clientType":"pc"}"#).unwrap();
    assert!(!params.is_empty());
}
