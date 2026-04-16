use aes::Aes128;
use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{BlockModeDecrypt, KeyInit};
use anyhow::anyhow;
use base64::Engine;
use derive_more::{Deref, Display, From, FromStr};
use ecb::Decryptor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnMedia {
    pub aes_key: Base64AesKey,
    pub encrypt_query_param: String,
    pub full_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Deref, From, FromStr, Display)]
pub struct HexAesKey(String);

#[derive(Debug, Clone, Serialize, Deserialize, Deref, From, FromStr, Display)]
pub struct Base64AesKey(String);
impl CdnMedia {
    pub async fn download(
        &self,
        http_client: &reqwest::Client,
        aes_key: Option<&HexAesKey>,
    ) -> crate::Result<Vec<u8>> {
        let aes_key = if let Some(aes_key) = aes_key {
            hex::decode(aes_key.as_bytes())?
        } else {
            let buf = base64::engine::general_purpose::STANDARD.decode(self.aes_key.as_bytes())?;
            match buf.len() {
                16 => buf,
                32 => {
                    let hex_str = std::str::from_utf8(&buf)?;
                    hex::decode(hex_str)?
                }
                other => return Err(anyhow!("unexpected aes_key len: {other}")),
            }
        };
        let ciphertext = http_client
            .get(&self.full_url)
            .send()
            .await?
            .bytes()
            .await?;
        let bytes = decrypt_aes_ecb(&aes_key, &ciphertext)?;
        Ok(bytes)
    }
}

fn decrypt_aes_ecb(aes_key: &[u8], ciphertext: &[u8]) -> crate::Result<Vec<u8>> {
    let dec =
        Decryptor::<Aes128>::new_from_slice(aes_key).map_err(|_| anyhow!("invalid aes key"))?;
    let mut buf = ciphertext.to_vec();
    let decrypted = dec
        .decrypt_padded::<Pkcs7>(&mut buf)
        .map_err(|_| anyhow!("aes ecb decrypt failed"))?;
    Ok(decrypted.to_vec())
}
