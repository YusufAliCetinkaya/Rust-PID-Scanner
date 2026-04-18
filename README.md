# Rust PID Scanner 
Windows süreçlerini ve PID numaralarını listeleyen, Rust tabanlı sistem aracı.

##  Özellikler
* **Hızlı ve Güvenli:** Rust'ın bellek güvenliği (memory safety) avantajlarını kullanır.
* **WinAPI Entegrasyonu:** `windows-sys` crate'i üzerinden doğrudan Windows API çağrıları gerçekleştirir.
* **Sade Çıktı:** PID ve Süreç Adı bilgilerini temiz bir tabloda sunar.

## Kullanılan Teknolojiler
* [Rust](https://www.rust-lang.org/)
* `windows-sys` (WinAPI bağlantısı için)
* `sysinfo` (Sistem bilgileri analizi için)

```bash
cargo run
