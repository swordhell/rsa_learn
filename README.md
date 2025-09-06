# 概述

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

RSA 签名与验证示例本项目旨在提供使用 Rust 和 Go 两种不同编程语言进行 RSA 签名与验证的完整示例。

它包含了签名、验证以及密钥文件处理的逻辑，帮助你理解非对称加密在实际应用中的工作流程。

特性

RSA 签名： 使用私钥对数据进行签名。

签名验证： 使用对应的公钥对签名进行验证。

多语言支持： 包含 Rust 和 Go 两种语言的实现。密钥格式： 支持解析 DER 格式的 PKCS#1 和 PKCS#8 两种密钥格式。

哈希算法： 示例中使用 MD5 哈希算法。文件结构这是一个典型的项目结构，包含 Rust 和 Go 的代码以及存放密钥的目录。

```txt
├── go_rsa_learn/
│   └── main.go
├── rust_rsa_learn/
│   └── src/
│       └── main.rs
│   └── Cargo.toml
└── keys/
    ├── rsa_private_key.der
    └── rsa_public_key.der
```

如何运行先决条件在运行代码之前，你需要确保已经安装了 Rust 和 Go。你还需要一个 RSA 私钥和一个对应的公钥文件。如果你没有，可以使用 OpenSSL 生成：# 生成私钥（PKCS#1 格式，2048位）

```bash
openssl genrsa -out rsa_private_key.pem 2048
```

# 将私钥转换为 DER 格式
```bash
openssl rsa -in rsa_private_key.pem -outform DER -out rsa_private_key.der
```

# 从私钥中提取公钥
```bash
openssl rsa -in rsa_private_key.pem -pubout -out rsa_public_key.pem
```

# 将公钥转换为 DER 格式

```bash
openssl rsa -pubin -in rsa_public_key.pem -outform DER -out rsa_public_key.der
请将生成的 rsa_private_key.der 和 rsa_public_key.der 文件放在 keys/ 目录下。运行 Rust 示例进入 rust_rsa_learn 目录：cd rust_rsa_learn
运行程序：cargo run
程序将执行签名和验证逻辑，并在终端输出结果。运行 Go 示例进入 go_rsa_learn 目录：cd go_rsa_learn
运行程序：go run main.go
```
