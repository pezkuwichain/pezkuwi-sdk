# PEZKUWICHAIN FRONTEND VPS DEPLOYMENT - TROUBLESHOOTING NOTES
**Date:** November 17, 2025  
**Environment:** VPS (37.60.230.9) - Ubuntu  
**Frontend URL:** https://pezkuwichain.io  
**Blockchain RPC:** wss://ws.pezkuwichain.io

---

## 🎯 OBJECTIVE
Deploy PezkuwiChain frontend to VPS and connect to blockchain validators running on same VPS.

---

## ❌ PROBLEM 1: WebSocket Connection Failed (Local Endpoint)

### Issue
Frontend trying to connect to `ws://127.0.0.1:9944` from user's browser, but validators are on VPS.
```
WebSocket connection to 'ws://127.0.0.1:9944/' failed: 1006:: Abnormal Closure
```

### Root Cause
Hardcoded `ws://127.0.0.1:9944` endpoints in multiple files:
- `src/components/ChainSpecs.tsx`
- `src/components/wallet/WalletModal.tsx`
- `src/components/MultisigMembers.tsx`
- `src/components/ReservesDashboard.tsx`
- `src/contexts/WebSocketContext.tsx`
- `src/App.tsx`

### Solution
Replace all local WebSocket endpoints with public WSS endpoint:
```bash
cd /var/www/pezkuwichain/web/web

# Replace hardcoded endpoints
sed -i "s|ws://127.0.0.1:9944|wss://ws.pezkuwichain.io|g" src/components/ChainSpecs.tsx
sed -i "s|ws://127.0.0.1:9945|wss://ws.pezkuwichain.io|g" src/components/ChainSpecs.tsx
sed -i "s|ws://127.0.0.1:9944|wss://ws.pezkuwichain.io|g" src/components/wallet/WalletModal.tsx
sed -i "s|ws://127.0.0.1:9944|wss://ws.pezkuwichain.io|g" src/components/MultisigMembers.tsx
sed -i "s|ws://127.0.0.1:9944|wss://ws.pezkuwichain.io|g" src/components/ReservesDashboard.tsx
sed -i "s|ws://127.0.0.1:9944|wss://ws.pezkuwichain.io|g" src/components/NetworkStats.tsx
sed -i "s|ws://127.0.0.1:9944|wss://ws.pezkuwichain.io|g" src/contexts/WebSocketContext.tsx

# Clean build
rm -rf dist/ node_modules/.vite
npm run build
```

### Key Learning
**Local vs Production endpoints:**
- **Local development:** `ws://127.0.0.1:9944` ✅ (validator on same machine)
- **VPS production:** `wss://ws.pezkuwichain.io` ✅ (validator remote, HTTPS required)

Browsers **cannot** connect to `ws://127.0.0.1:9944` when validator is on remote VPS.

---

## ❌ PROBLEM 2: Supabase Configuration Missing

### Issue
```
Uncaught Error: supabaseUrl is required.
```

### Root Cause
`.env` file not present in VPS build directory or not loaded during build.

### Solution
Create `.env` file in `/var/www/pezkuwichain/web/web/`:
```bash
cat > .env << 'EOF'
# VPS Production Config
VITE_NETWORK=testnet

# Blockchain WebSocket
VITE_CHAIN_ENDPOINT=wss://ws.pezkuwichain.io
VITE_WS_ENDPOINT_LOCAL=wss://ws.pezkuwichain.io
VITE_WS_ENDPOINT_TESTNET=wss://ws.pezkuwichain.io

# Chain Configuration
VITE_CHAIN_NAME=PezkuwiChain
VITE_CHAIN_TOKEN_SYMBOL=HEZ
VITE_CHAIN_TOKEN_DECIMALS=12
VITE_CHAIN_SS58_FORMAT=42

# Asset IDs
VITE_ASSET_WHEZ=0
VITE_ASSET_PEZ=1

# Supabase
VITE_SUPABASE_URL=https://vsyrpfiwhjvahofxwytr.supabase.co
VITE_SUPABASE_ANON_KEY=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InZzeXJwZml3aGp2YWhvZnh3eXRyIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NjAwMjYxNTgsImV4cCI6MjA3NTYwMjE1OH0.dO2c8YWIph2D95X7jFdlGYJ8MXyuyorkLcjQ6onH-HE
EOF

# Rebuild
npm run build
```

### Key Learning
Vite requires `.env` file **before build** to embed environment variables. Variables starting with `VITE_` are exposed to frontend.

---

## ❌ PROBLEM 3: Nginx 404 Not Found

### Issue
Browser shows `404 Not Found` when accessing `http://37.60.230.9`.

### Root Cause
Nginx configured for domain name (`pezkuwichain.io`) only, not for direct IP access.

### Solution
**Option 1:** Use domain (recommended)
```
https://pezkuwichain.io
```

**Option 2:** Create default server for IP access
```bash
cat > /etc/nginx/sites-available/default-ip << 'EOF'
server {
    listen 80 default_server;
    listen [::]:80 default_server;
    
    root /var/www/pezkuwichain/web/dist;
    index index.html;
    
    location / {
        try_files $uri $uri/ /index.html;
    }
}
EOF

ln -sf /etc/nginx/sites-available/default-ip /etc/nginx/sites-enabled/
nginx -t && systemctl reload nginx
```

### Key Learning
Production deployments should use **domain names with SSL**, not raw IP addresses.

---

## ❌ PROBLEM 4: Git Ownership Error

### Issue
```
fatal: detected dubious ownership in repository at '/var/www/pezkuwichain/web'
```

### Root Cause
Directory owned by user `1000:1000`, but Git commands run as `root`.

### Solution
```bash
git config --global --add safe.directory /var/www/pezkuwichain/web
```

### Key Learning
VPS directories may have different ownership. Add to Git safe directories when needed.

---

## ❌ PROBLEM 5: Nested Directory Structure Confusion

### Issue
`package.json` not found in `/var/www/pezkuwichain/web`.

### Root Cause
Project has nested structure:
```
/var/www/pezkuwichain/
  ├── web/              (parent)
  │   ├── web/          (actual project)
  │   │   ├── package.json ✅
  │   │   ├── src/
  │   │   └── dist/
  │   ├── mobile/
  │   └── shared/
```

### Solution
Always work in correct directory:
```bash
cd /var/www/pezkuwichain/web/web  # ← Actual project
npm install
npm run build

# Copy build to parent for Nginx
rm -rf ../dist
cp -r dist ../
```

### Key Learning
Monorepo structure requires attention to correct working directory. VPS deployment used nested `web/web/` structure.

---

## ❌ PROBLEM 6: SSL Certificate Missing for Subdomains

### Issue
Subdomains like `beta.pezkuwichain.io`, `testnet.pezkuwichain.io` not accessible via HTTPS.

### Root Cause
SSL certificates only existed for:
- `pezkuwichain.io`
- `ws.pezkuwichain.io`
- `api.pezkuwichain.io`

Missing for: `beta`, `testnet`, `staging`, `mainnet`, `explorer`

### Solution
Request SSL certificates for all subdomains:
```bash
certbot certonly --nginx \
  -d beta.pezkuwichain.io \
  -d testnet.pezkuwichain.io \
  -d staging.pezkuwichain.io \
  -d mainnet.pezkuwichain.io \
  -d explorer.pezkuwichain.io \
  --non-interactive --agree-tos --email satoshiqazi@gmail.com
```

Create Nginx configs for each:
```bash
# Beta (Validator 1 - Port 9944)
cat > /etc/nginx/sites-available/beta.pezkuwichain.io.conf << 'EOF'
server {
    server_name beta.pezkuwichain.io;
    
    location / {
        proxy_pass http://127.0.0.1:9944;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    listen 443 ssl;
    ssl_certificate /etc/letsencrypt/live/beta.pezkuwichain.io/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/beta.pezkuwichain.io/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;
}
EOF

ln -sf /etc/nginx/sites-available/beta.pezkuwichain.io.conf /etc/nginx/sites-enabled/
nginx -t && systemctl reload nginx
```

### Key Learning
Production blockchain deployments need SSL for **all** RPC endpoints to enable secure WebSocket connections (WSS).

---

## ❌ PROBLEM 7: Build Cache Causing Stale Endpoints

### Issue
After replacing endpoints, build still contains `ws://127.0.0.1:9944`.

### Root Cause
Vite caches build artifacts in:
- `dist/` directory
- `node_modules/.vite/` directory

### Solution
**Clean rebuild process:**
```bash
# Remove all caches
rm -rf dist/ node_modules/.vite .vite

# Rebuild from scratch
npm run build

# Verify no local endpoints in build
grep -r "ws://127.0.0.1" dist/ && echo "❌ Still has local" || echo "✅ Clean"
```

### Key Learning
Always **purge cache** when changing environment variables or critical endpoints. Otherwise old values persist in build.

---

## ❌ PROBLEM 8: i18n Translations Not Loading

### Issue
Translation keys displayed as raw text: `delegation.title` instead of "Vote Delegation"

### Root Cause
i18n config importing from `@pezkuwi/i18n` shared package which lacked `delegation` keys.

### Solution
Use local translation files instead:
```typescript
// Before (broken)
import { translations } from '@pezkuwi/i18n';

// After (working)
import en from './locales/en';
import tr from './locales/tr';
// ... etc

const resources = {
  en: { translation: en },
  tr: { translation: tr },
  // ...
};
```

### Key Learning
Shared packages may not have complete translations. Use **local translation files** for frontend-specific keys.

---

## ✅ SUCCESSFUL DEPLOYMENT CHECKLIST

### 1. SSL Certificates
```bash
# Check all certificates
ls -la /etc/letsencrypt/live/ | grep pezkuwichain

# Should have:
# - pezkuwichain.io
# - ws.pezkuwichain.io
# - beta.pezkuwichain.io
# - testnet.pezkuwichain.io
# - staging.pezkuwichain.io
# - mainnet.pezkuwichain.io
# - explorer.pezkuwichain.io
```

### 2. Nginx Configurations
```bash
# Check all sites enabled
ls -la /etc/nginx/sites-enabled/ | grep pezkuwichain

# Test config
nginx -t

# Reload
systemctl reload nginx
```

### 3. Environment Variables
```bash
cd /var/www/pezkuwichain/web/web

# Verify .env exists
cat .env | grep VITE_SUPABASE_URL
cat .env | grep VITE_CHAIN_ENDPOINT

# Should show:
# VITE_CHAIN_ENDPOINT=wss://ws.pezkuwichain.io
# VITE_SUPABASE_URL=https://vsyrpfiwhjvahofxwytr.supabase.co
```

### 4. Clean Build
```bash
# Remove caches
rm -rf dist/ node_modules/.vite

# Build
npm run build

# Verify no local endpoints
grep -r "ws://127.0.0.1" dist/ && echo "❌ Has local" || echo "✅ Clean"
```

### 5. Copy to Nginx Root
```bash
# Copy build to parent directory
rm -rf ../dist
cp -r dist ../

# Verify Nginx can read it
ls -la /var/www/pezkuwichain/web/dist/index.html
```

### 6. Test in Browser
- **URL:** https://pezkuwichain.io
- **Check Console:** Should see "Connected to wss://ws.pezkuwichain.io"
- **Check Network Status:** Should show "Connected" (green)
- **Check Block Height:** Should increment every 6 seconds

---

## 🎯 FINAL WORKING CONFIGURATION

### VPS Structure
```
/var/www/pezkuwichain/
├── backend/              (API - Port 8000)
├── web/
│   ├── dist/            (Nginx serves from here)
│   └── web/             (Source code & build)
│       ├── src/
│       ├── package.json
│       ├── .env         (Production config)
│       └── dist/        (Built, then copied to ../dist)
└── mobile/
```

### Nginx Main Config
```nginx
# /etc/nginx/sites-available/pezkuwichain.io
server {
    server_name pezkuwichain.io www.pezkuwichain.io;
    root /var/www/pezkuwichain/web/dist;  # ← Parent dist
    index index.html;
    
    location / {
        try_files $uri $uri/ /index.html;
    }
    
    location /api/ {
        proxy_pass http://127.0.0.1:8000/;
        # ... proxy headers
    }
    
    listen 443 ssl;
    ssl_certificate /etc/letsencrypt/live/pezkuwichain.io/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/pezkuwichain.io/privkey.pem;
}
```

### Environment Variables (.env)
```bash
VITE_NETWORK=testnet
VITE_CHAIN_ENDPOINT=wss://ws.pezkuwichain.io
VITE_WS_ENDPOINT_LOCAL=wss://ws.pezkuwichain.io
VITE_WS_ENDPOINT_TESTNET=wss://ws.pezkuwichain.io
VITE_SUPABASE_URL=https://vsyrpfiwhjvahofxwytr.supabase.co
VITE_SUPABASE_ANON_KEY=<your-key>
```

---

## 🚀 DEPLOYMENT WORKFLOW

### Standard Deployment Process
```bash
# 1. SSH to VPS
ssh pezkuwi-vps

# 2. Navigate to project
cd /var/www/pezkuwichain/web/web

# 3. Pull latest code (if using Git)
git pull origin main

# 4. Install dependencies (if package.json changed)
npm install

# 5. Clean build
rm -rf dist/ node_modules/.vite
npm run build

# 6. Copy to Nginx root
rm -rf ../dist
cp -r dist ../

# 7. Reload Nginx
systemctl reload nginx

# 8. Test
curl -I https://pezkuwichain.io
```

### Quick Update (No dependencies changed)
```bash
ssh pezkuwi-vps
cd /var/www/pezkuwichain/web/web
rm -rf dist/
npm run build
cp -r dist ../
systemctl reload nginx
```

---

## 📊 PERFORMANCE METRICS

### Build Stats
```
Build Time: ~20 seconds
Output Size: 4.5MB (gzipped)
Bundle: index-*.js (main)
Assets: PNG, SVG, PDF
```

### Network Performance
```
Blockchain RPC: wss://ws.pezkuwichain.io
Latency: <50ms (local VPS)
Block Time: 6 seconds
Active Connections: 8 validators
```

---

## 🔧 TROUBLESHOOTING COMMANDS

### Check Frontend Status
```bash
# Nginx logs
tail -f /var/log/nginx/pezkuwichain.io.access.log
tail -f /var/log/nginx/pezkuwichain.io.error.log

# Check if site is up
curl -I https://pezkuwichain.io

# Check WebSocket endpoint
curl -I https://ws.pezkuwichain.io
```

### Check Blockchain Connection
```bash
# Test RPC endpoint
wscat -c wss://ws.pezkuwichain.io

# Should receive:
# Connected (press CTRL+C to quit)

# Check validator logs
journalctl -u pezkuwi-validator-1 -f
```

### Check Build Issues
```bash
cd /var/www/pezkuwichain/web/web

# Check for hardcoded local endpoints
grep -r "ws://127.0.0.1" src/

# Check environment variables loaded
cat .env | grep VITE_

# Verify build output
ls -lh dist/
```

---

## 🎓 KEY LESSONS LEARNED

1. **WebSocket Endpoints:** Always use `wss://` for remote validators, `ws://127.0.0.1` only for local development.

2. **Environment Variables:** Vite requires `.env` file **before build**. Variables must start with `VITE_` prefix.

3. **Build Caching:** Always purge cache (`rm -rf dist/ node_modules/.vite`) when changing critical configs.

4. **SSL Certificates:** Production blockchain requires SSL for **all** RPC endpoints to enable secure WebSocket (WSS).

5. **Nginx Structure:** Nested directory structure requires careful path management. Build output must be copied to Nginx root.

6. **i18n Configuration:** Use local translation files for frontend-specific keys, not shared packages.

7. **Git Ownership:** Add VPS directories to Git safe directories when ownership differs.

8. **Domain vs IP:** Always use domain names with SSL in production, not raw IP addresses.

---

## ✅ VERIFICATION CHECKLIST

Before declaring deployment successful, verify:

- [ ] Frontend loads at https://pezkuwichain.io
- [ ] Console shows: "Connected to wss://ws.pezkuwichain.io"
- [ ] Network Status widget shows "Connected" (green)
- [ ] Block height increments every 6 seconds
- [ ] Polkadot.js wallet connects successfully
- [ ] Balance data loads (HEZ, PEZ, wHEZ, wUSDT)
- [ ] P2P platform accessible at /p2p
- [ ] Education platform accessible
- [ ] i18n translations working (no raw keys)
- [ ] No console errors related to WebSocket
- [ ] No console errors related to Supabase

---

## 📝 NOTES FOR FUTURE DEPLOYMENTS

### DO:
✅ Always use domain names with SSL  
✅ Purge build cache when changing endpoints  
✅ Test WebSocket connection in browser console  
✅ Verify environment variables before build  
✅ Copy built files to Nginx root directory  
✅ Check Nginx logs after deployment  

### DON'T:
❌ Use `ws://127.0.0.1` for remote validators  
❌ Commit `.env` files to Git  
❌ Skip SSL certificate verification  
❌ Forget to reload Nginx after config changes  
❌ Deploy without testing in browser first  
❌ Use raw IP addresses in production  

---

**Document Version:** 1.0  
**Last Updated:** November 17, 2025  
**Status:** ✅ Deployment Successful  
**Production URL:** https://pezkuwichain.io