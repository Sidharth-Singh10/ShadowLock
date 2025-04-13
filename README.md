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

### Development 

add `PASSWORD_SALT` in your environment or in `.env` file and then run `cargo run`

### Actual Usage

add `PASSWORD_SALT` in your environment or in `.env` file 
Run
1) `cargo build --release`
2) `cargo install --path .`


current state of application:

![Screenshot_20250414_023342](https://github.com/user-attachments/assets/7f9f5168-ef2e-4165-adc2-36db523a76eb)
![Screenshot_20250414_023327](https://github.com/user-attachments/assets/a77a84c7-18d6-493c-a721-c052d856f324)
