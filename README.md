# ULTRAXRAY
**ULTRAXRAY-lite**
---
Kenapa yang versi lite butuh DNS local + fake dns, tanpa dns local,tanpa fake dns akan jadi "netERR_NAME_NOT_RESOLVE".
Jawaban nya adalah: 

**Local Fake DNS**

Karena di client (entah itu perangkat, router, atau VPN) query DNS default akan tetap ke 8.8.8.8, 1.1.1.1, atau DNS ISP.
Kamu perlu fake DNS agar:
Semua DNS UDP disedot ke proxy (misal melalui tun2socks / redsocks / transparent proxy DNS hijack).
Diteruskan ke backend Worker (pakai DoH seperti fungsi doh() ini).
Kalau tidak:
DNS tetap dilewatkan ke upstream biasa.
Client tetap dapat net::ERR_NAME_NOT_RESOLVED karena tidak ada resolve melalui tunnel.

---

**Integrasi**

Di sisi VPN atau tunnel-mu (misal pakai tun2socks atau proxy websocket):
Harus intercept DNS (UDP:53) dan arahkan ke local fake DNS (misal 127.0.0.1:5353).
Fake DNS kemudian forward via WebSocket -> Worker -> DoH.

---

**ULTRAXRAY-pro**

abc

---

# Siren

**A Serverless V2Ray Tunnel Optimized for Indonesia**

Siren is a lightweight and serverless V2Ray tunnel built on [Cloudflare Workers](https://workers.cloudflare.com/), supporting modern proxy protocols.  
It offers fast, secure, and scalable deployment without the need for a traditional VPS.

---

## 🔧 Features

- ✅ **Multi-Protocol Support**

  - VMess
  - Trojan
  - VLESS
  - Shadowsocks

- ✅ **Domain over HTTPS (DoH)**  
  Encrypts DNS queries for improved privacy and security.

---

## 🌐 Endpoints

| Endpoint | Description                       |
| -------- | --------------------------------- |
| `/`      | Main landing page                 |
| `/link`  | Generate shareable proxy links    |
| `/sub`   | Subscription endpoint for clients |

---

## 🚀 Deployment Guide

Siren can be deployed seamlessly using GitHub Actions with Cloudflare Workers.

### ⚙️ CI/CD via GitHub Actions

1. **Create a KV Namespace**

   - Go to Cloudflare Dashboard → Workers → KV.
   - Create a new namespace named `SIREN`.

2. **Configure `wrangler.toml`**

   - Add the KV namespace to your config file:
     ```toml
     [[kv_namespaces]]
     binding = "SIREN"
     id = "YOUR_KV_NAMESPACE_ID"
     ```

3. **Generate API Token**

   - [Create an API Token](https://developers.cloudflare.com/fundamentals/api/get-started/create-token/) with:
     - Permissions: Workers & KV Storage

4. **Set GitHub Repository Secret**

   - Navigate to: GitHub → Your Repo → Settings → Secrets and variables → Actions
   - Add a new secret:
     - Name: `CLOUDFLARE_API_TOKEN`
     - Value: Your API token

5. **Enable GitHub Actions**

   - Open the **Actions** tab on GitHub.
   - Enable workflows if prompted.

6. **Trigger Deployment**

   - Push any commit or manually trigger the deployment workflow.

7. **Access Your Siren Instance**
   - Visit: `https://<YOUR-WORKERS-SUBDOMAIN>.workers.dev`
