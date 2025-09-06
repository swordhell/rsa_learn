use anyhow::Result;
use base64::Engine;
use base64::engine::general_purpose;
use base64::prelude::*;
use rand::thread_rng;
use rsa::RsaPrivateKey;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::{RsaPublicKey, pkcs1v15::Pkcs1v15Sign};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

// 导入 Digest trait，以便使用 Md5 哈希函数
use digest::Digest;
use md5::Md5;
use rsa::pkcs1::DecodeRsaPublicKey;

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
    let signature =
        private_key.sign_with_rng(&mut rng, Pkcs1v15Sign::new::<Md5>(), &hashed_data)?;

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

    // // 读取 PEM 文件内容
    let pub_base64 = fs::read_to_string("flatten_rsa_public_key.pem")?;
    let pub_der_bytes = general_purpose::STANDARD.decode(pub_base64.trim())?;
    let public_key_load = parse_rsa_public_key(&pub_der_bytes)?;
    println!("Public key loaded: {:?}", public_key_load);
    println!("Public key parsed successfully from PEM file.");

    let public_key = private_key.to_public_key();

    // 使用公钥验证签名，传入哈希过的数据和签名
    let padding = Pkcs1v15Sign::new::<md5::Md5>();
    match public_key.verify(padding, &hashed_data, &signature) {
        Ok(_) => println!("签名验证成功！"),
        Err(e) => println!("签名验证失败：{}", e),
    }
     let padding_oth = Pkcs1v15Sign::new::<md5::Md5>();
    match public_key_load.verify(padding_oth, &hashed_data, &signature) {
        Ok(_) => println!("签名验证2成功！"),
        Err(e) => println!("签名验证2失败：{}", e),
    }

    println!("--- 验证签名结束 ---");
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
    Err(anyhow::anyhow!(
        "Failed to parse DER: neither PKCS#1 nor PKCS#8"
    ))
}

// 解析 DER 格式的 RSA 公钥（支持 PKCS#1 和 PKCS#8）
fn parse_rsa_public_key(der: &[u8]) -> Result<RsaPublicKey> {
    if let Ok(public_key) = RsaPublicKey::from_pkcs1_der(der) {
        return Ok(public_key);
    }
     // 尝试 PKCS#8 格式
    if let Ok(pk8) = RsaPublicKey::from_public_key_der(der) {
        return Ok(pk8);
    }
    Err(anyhow::anyhow!(
        "Failed to parse public key DER: neither PKCS#1 nor PKCS#8"
    ))
}
