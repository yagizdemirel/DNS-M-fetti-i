<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/1/1a/Istinye_University_logo.svg/1024px-Istinye_University_logo.svg.png" alt="İstinye Üniversitesi Logo" width="250"/>

  <h1>🕵️ DNS Müfettişi (DNS Inspector)</h1>
  
  <p><strong>Ağ trafiğindeki DNS sorgularını yakalayan, analiz eden ve Güvenli DNS (DoH - DNS over HTTPS) üzerinden çözümleyen yüksek performanslı yerel araç.</strong></p>

  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/Axum-v0.7-blue?style=for-the-badge" alt="Axum" />
  <img src="https://img.shields.io/badge/Tokio-Asynchronous-yellow?style=for-the-badge" alt="Tokio" />
  <img src="https://img.shields.io/badge/Cloudflare-DoH-F38020?style=for-the-badge&logo=cloudflare&logoColor=white" alt="Cloudflare DoH" />
  <img src="https://img.shields.io/badge/Status-Active-success?style=for-the-badge" alt="Status" />

</div>

<hr />

## 📋 Proje Bilgileri

* **Kurum:** İstinye Üniversitesi
* **Geliştirici:** Yağız Veysel
* **Danışman / Eğitmen:** [Eğitmen/Danışman Adı Soyadı]
* **Geliştirme Dili:** Rust
* **Mimari:** Modüler, Asenkron (Event-driven)

---

## 📑 İçindekiler
1. [Proje Özeti ve Amacı](#-proje-özeti-ve-amacı)
2. [Teknik Mimari ve Derinlik](#-teknik-mimari-ve-derinlik)
3. [Özellikler](#-özellikler)
4. [Kurulum ve Çalıştırma](#-kurulum-ve-çalıştırma)
5. [Görsel Arayüz (Dashboard)](#-görsel-arayüz-dashboard)
6. [Güvenlik ve Gizlilik (Savunma Mekanizması)](#-güvenlik-ve-gizlilik-savunma-mekanizması)

---

## 🎯 Proje Özeti ve Amacı

**DNS Müfettişi**, sistemdeki DNS isteklerini (Port 53) araya girerek yakalayan ve bu istekleri standart (ve güvensiz) UDP protokolü yerine **Cloudflare DoH (1.1.1.1)** kullanarak HTTPS üzerinden gizlice şifreleyerek çözen bir güvenlik ve analiz aracıdır.

*"Telefondaki rehberden kime bakıldığını (DNS) gizlice dinle. Kullanıcının hangi adresleri aradığını bul ama çözümlemeyi HTTPS üzerinden dışarıya sızmadan güvenle yap."* felsefesiyle tasarlanmıştır.

---

## 🏗️ Teknik Mimari ve Derinlik

Sistem, bakım edilebilirliği (maintainability) artırmak ve *Solid* prensiplerine uymak amacıyla modüler bir yapıya bölünmüştür:

* `src/main.rs`: Uygulamanın giriş noktası (Entry point) ve thread yönetimi.
* `src/dns.rs`: Hickory tabanlı DNS sunucusu, paket yakalama ve DoH çözümleme motoru.
* `src/web.rs`: Axum tabanlı asenkron REST API ve statik dosya sunucusu.
* `src/models.rs`: Veri yapıları ve Serde ile uçtan uca veri serileştirme.

---

## ✨ Özellikler

* **Gerçek Zamanlı Yakalama:** Gelen tüm DNS (A, AAAA, MX, TXT vb.) sorgularını anında tespit eder.
* **DNS over HTTPS (DoH):** Yakalanan istekleri İSS (Servis Sağlayıcı) takibinden kaçırmak için TLS üzerinden şifreleyerek çözer.
* **Asenkron Performans:** `Tokio` çalışma zamanı sayesinde binlerce eşzamanlı isteği bloklanmadan (non-blocking) işler.
* **Modern Dashboard:** Glassmorphism tasarımlı, gerçek zamanlı güncellenen (Polling) web arayüzü.

---

## 🚀 Kurulum ve Çalıştırma

### Gereksinimler
* [Rust](https://www.rust-lang.org/tools/install) (1.70+ önerilir)
* UDP Port 5300 ve TCP Port 3000'in kullanımda olmaması.

### Adımlar

1. Repoyu klonlayın:
   ```bash
   git clone <repo-url>
   cd dns-inspector
