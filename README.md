# 🔐 ShadowLock

**ShadowLock** is a simple and secure deterministic **stateless** password manager built in Rust.  
It derives your site-specific passwords using a master password, a site name, and a pre-password, without ever storing anything on disk.

A pre-password is a small memorable password which is used to derive your acutal password for your site  

---

## ✨ Features

- 🧠 Deterministic password generation — generate the same password every time using the same inputs
- 🛡️ Uses **Argon2** for master key hashing (resistant to GPU/ASIC cracking)
- 🔑 BLAKE3 used for fast keyed site+pre-password hashing
- 🔒 No storage — nothing is written to disk
- 🔐 Output passwords include a mix of ASCII printable characters

---

## 🧪 Example Usage

```bash
PASSWORD_SALT=mysalt cargo run -- <MASTER_PASSWORD> <SITE_NAME> <PRE_PASSWORD>
```

current state of application:
![image2](https://github.com/user-attachments/assets/1dbf1fe5-59cf-4a61-8616-99184055c85f)
