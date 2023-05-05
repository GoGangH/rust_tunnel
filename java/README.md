JSch는 reverse tunnel을 구현할수 있게 해주는 java라이브러리이다.

## 자바에 적용하는 방법
## 코드별 주석 작성 필요

The current JSch has the following features.

JSch is in pure Java, but it depends on JavaTM Cryptography Extension (JCE). JSch has been known to work with:
J2SE 1.4.0 or later (no additional libraries required).
J2SE 1.3 and Sun's JCE reference implementation that can be obtained at http://java.sun.com/products/jce/.
J2SE 1.2.2 and later and Bouncycastle's JCE implementation that can be obtained at http://www.bouncycastle.org/.
SSH2 protocol support.
Key exchange: diffie-hellman-group-exchange-sha1,diffie-hellman-group1-sha1,diffie-hellman-group14-sha1,diffie-hellman-group-exchange-sha256,ecdh-sha2-nistp256,ecdh-sha2-nistp384,ecdh-sha2-nistp521
Cipher: blowfish-cbc,3des-cbc,aes128-cbc,aes192-cbc,aes256-cbc,aes128-ctr,aes192-ctr,aes256-ctr,3des-ctr,arcfour,arcfour128,arcfour256
MAC: hmac-md5, hmac-sha1, hmac-md5-96, hmac-sha1-96
Host key type: ssh-dss,ssh-rsa,ecdsa-sha2-nistp256,ecdsa-sha2-nistp384,ecdsa-sha2-nistp521
Userauth: password
Userauth: publickey(DSA,RSA,ECDSA)
Userauth: keyboard-interactive
Userauth: gssapi-with-mic
X11 forwarding
xauth spoofing
connection through HTTP proxy.
connection through SOCKS5 proxy.
port forwarding.
stream forwarding.
signal sending. The unofficial patch for sshd of openssh will be found in this thread.
envrironment variable passing.
remote exec.
generating DSA and RSA key pairs.
supporting private keys in OpenSSL(traditional SSLeay) and PKCS#8 format.
changing the passphrase for a private key.
partial authentication
SSH File Transfer Protocol(version 0, 1, 2, 3)
packet compression: zlib, zlib@openssh.com JZlib has been used.
hashed known_hosts file.
NONE Cipher switching. High Performace Enabled SSH/SCP supports NONE Cipher switching. Refer to ScpToNoneCipher.java.
JSch is licensed under BSD style license.