# 概述

生成密钥

```bash
openssl genrsa -out rsa_private_key.pem 2048
openssl rsa -in rsa_private_key.pem -out rsa_public_key.pem -pubout
# 将打平的私钥输出
echo "<your-flatten-rsa-private-key>" | base64 -d | openssl pkey -inform DER -text
# 如果是 PKCS#8 PEM，生成der文件；
openssl pkcs8 -topk8 -inform PEM -outform DER -in rsa_private_key.pem -out rsa_private_key.der -nocrypt
# 如果是 PKCS#1 PEM
openssl rsa -inform PEM -outform DER -in rsa_private_key.pem -out rsa_private_key.der
# 查看key
openssl pkey -in rsa_private_key.pem -text -noout
openssl pkey -pubin -in rsa_public_key.pem -text -noout
# 打平私钥/公钥
grep -v "-----" key.pem | tr -d '\n'
```

```pem
# PKCS#1 RSA 私钥
-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAsd...
-----END RSA PRIVATE KEY-----

# PKCS#8 私钥（通用格式）
-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCx...
-----END PRIVATE KEY-----
```

```bash
echo -n "OpenId=HASH13900000002" > data.txt
openssl dgst -md5 -sign rsa_private_key.der -out signature.bin data.txt
openssl base64 -in signature.bin -out signature.b64
# linux/mac里面有命令来处理
base64 signature.bin > signature.b64
```