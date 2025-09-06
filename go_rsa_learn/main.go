package main

import (
	"crypto"
	"crypto/md5"
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/base64"
	"fmt"
	"io"
	"log"
	"os"
)

func main() {
    // 读取 DER 文件
    derBytes, err := os.ReadFile("rsa_private_key.der")
    if err != nil {
        log.Fatalf("Failed to read DER file: %v", err)
    }

    // 尝试解析 DER 数据
    privateKey, err := parseRSAPrivateKey(derBytes)
    if err != nil {
        log.Fatalf("Failed to parse private key: %v", err)
    }

    // 打印密钥信息（示例）
    fmt.Printf("RSA Private Key:\n")
    fmt.Printf("Modulus (N): %x\n", privateKey.N)
    fmt.Printf("Private Exponent (D): %x\n", privateKey.D)

	rawStr := "The grass is always greener on the other side."

	hash := md5.New()

	io.WriteString(hash, rawStr)
	hashInBytes := hash.Sum(nil)

	signature, err := rsa.SignPKCS1v15(rand.Reader, privateKey, crypto.MD5, hashInBytes[:])
	if err != nil {
		return 
	}

	signatureBase64  := base64.StdEncoding.EncodeToString(signature)
	fmt.Println("Signature (base64):", signatureBase64)
}

// parseRSAPrivateKey 尝试解析 DER 编码的 RSA 私钥（支持 PKCS#1 和 PKCS#8）
func parseRSAPrivateKey(der []byte) (*rsa.PrivateKey, error) {
    // 尝试解析 PKCS#8 格式
    key, err := x509.ParsePKCS8PrivateKey(der)
    if err == nil {
        rsaKey, ok := key.(*rsa.PrivateKey)
        if !ok {
            return nil, fmt.Errorf("PKCS#8 contains non-RSA private key")
        }
        return rsaKey, nil
    }

    // 如果 PKCS#8 解析失败，尝试 PKCS#1 格式
    rsaKey, err := x509.ParsePKCS1PrivateKey(der)
    if err == nil {
        return rsaKey, nil
    }

    return nil, fmt.Errorf("failed to parse DER: neither PKCS#1 nor PKCS#8: %v", err)
}