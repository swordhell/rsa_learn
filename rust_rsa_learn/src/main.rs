use anyhow::Result;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs1v15::Pkcs1v15Sign;
use rsa::pkcs8::DecodePrivateKey;
use rsa::RsaPrivateKey;
use std::fs::File;
use std::io::{Read, Write};

use base64::prelude::*;
use rand::thread_rng;

// 导入 Digest trait，以便使用 Md5 哈希函数
use digest::Digest;
use md5::Md5;

fn main() -> Result<()> {
    // 读取 DER 格式的私钥文件
    let mut der_file = File::open("rsa_private_key.der")?;
    let mut der_bytes = Vec::new();
    der_file.read_to_end(&mut der_bytes)?;

    // 解析私钥
    let private_key = parse_rsa_private_key(&der_bytes)?;
    
    // 待签名数据，这句谚语的意思是“这山还望那山高”
    let data = b"The grass is always greener on the other side.";

    // 创建一个随机数生成器
    let mut rng = thread_rng();

    // 手动对数据进行 MD5 哈希
    let mut hasher = Md5::new();
    hasher.update(data);
    let hashed_data = hasher.finalize();

    // 输出计算出的哈希值
    println!("计算出的哈希值: {:x?}", hashed_data);

    // 使用 RsaPrivateKey 上的公共 API sign_with_rng。
    // Pkcs1v15Sign 签名器现在接收的是哈希过的数据。
    let signature = private_key.sign_with_rng(
        &mut rng,
        Pkcs1v15Sign::new::<Md5>(),
        &hashed_data,
    )?;

    // 保存二进制签名
    let mut sig_file = File::create("signature.bin")?;
    sig_file.write_all(&signature)?;
    println!("Signature saved to signature.bin");

    // 保存 Base64 签名
    let base64_signature = BASE64_STANDARD.encode(&signature);
    let mut base64_file = File::create("signature.b64")?;
    base64_file.write_all(base64_signature.as_bytes())?;
    println!("Base64 signature saved to signature.b64");
    println!("Base64 signature: {}", base64_signature);

    Ok(())
}

// 解析 DER 格式的 RSA 私钥（支持 PKCS#1 和 PKCS#8）
fn parse_rsa_private_key(der: &[u8]) -> Result<RsaPrivateKey> {
    if let Ok(private_key) = RsaPrivateKey::from_pkcs8_der(der) {
        return Ok(private_key);
    }
    if let Ok(private_key) = RsaPrivateKey::from_pkcs1_der(der) {
        return Ok(private_key);
    }
    Err(anyhow::anyhow!("Failed to parse DER: neither PKCS#1 nor PKCS#8"))
}
