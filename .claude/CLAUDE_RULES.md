# Claude Kuralları - Pezkuwi SDK

Bu dosya her oturumda Claude tarafından okunmalı ve kurallara kesinlikle uyulmalıdır.

---

## ANA HEDEF

**Rebrand edilmiş Pezkuwi SDK projesinin `cargo check` komutundan `Finished` çıktısı alması.**

Bu hedef iki parçadan oluşur:
1. Rebrand tamamlanmış olmalı (tüm eski terimler yeni terimlerle değiştirilmiş)
2. Proje derlenebilir durumda olmalı (cargo check Finished)

**HER İKİ KOŞUL DA SAĞLANMALI. BİRİ DİĞERİ İÇİN FEDAKARİLİK EDİLEMEZ.**

---

## DEĞİŞMEZ KURALLAR - ASLA YAPMA

### 1. Rebrand'i Geri Alma Yasağı
- **ASLA** rebrand edilmiş kodu eski haline döndürme
- **ASLA** cargo check hataları için isim değişikliklerini geri alma
- **ASLA** "çalışsın diye" terminolojiyi bozma
- **ASLA** pezkuwi → polkadot, bizinikiwi → substrate gibi geri dönüşüm yapma

### 2. Terminoloji Koruma
Aşağıdaki dönüşümler kalıcıdır ve geri alınamaz:

| Eski (KULLANMA) | Yeni (KULLAN) |
|-----------------|---------------|
| polkadot | pezkuwi |
| Polkadot | Pezkuwi |
| substrate | bizinikiwi |
| Substrate | Bizinikiwi |
| cumulus | pezcumulus |
| Cumulus | Pezcumulus |
| westend | zagros |
| Westend | Zagros |
| rococo | pezkuwichain |
| Rococo | Pezkuwichain |
| parachain | teyrchain |
| Parachain | Teyrchain |
| paritytech | pezkuwichain |
| frame- | pezframe- |
| pallet- | pezpallet- |
| sp- | pezsp- |
| sc- | pezsc- |

### 3. Hata Çözme Yaklaşımı
Cargo check hatası aldığında:
- **DOĞRU:** Hatayı rebrand'e uygun şekilde düzelt (yeni terminoloji kullan)
- **YANLIŞ:** Rebrand'i geri alarak hatayı "çöz"

Örnek:
```
Hata: pezsp_runtime bulunamadı
DOĞRU: pezsp_runtime modülünü/import'unu düzelt
YANLIŞ: sp_runtime'a geri dön
```

---

## ÇALIŞMA PRENSİPLERİ

### 1. Checkpoint Sistemi
- Her büyük işlemden önce git commit yap
- Her hata düzeltmesinde "ne yaptım" raporu ver
- Geri dönüşü olmayan değişiklikler için onay iste

### 2. Audit Trail
- Değişiklik yapılan dosyaları listele
- Her oturum sonunda özet rapor sun

### 3. Şeffaflık
- Yapılan her değişikliği kullanıcıya bildir
- Varsayımda bulunma, sor
- Kısayol arama, doğru yolu izle

---

## YASAK DAVRANIŞLAR

1. Kullanıcının talimatı olmadan rebrand'i geri alma
2. "Derleme için gerekli" bahanesiyle terminolojiyi bozma
3. Uzun süren hatalar için "kolay çözüm" olarak geri dönüş yapma
4. Commit mesajına Claude imzası ekleme
5. Test etmeden commit/push yapma

---

## HATA DURUMUNDA

Eğer cargo check 100+ hata veriyorsa:
1. Panik yapma
2. Hataları kategorize et
3. Sistematik olarak düzelt (rebrand'e uygun)
4. Her 10-20 hata düzeltmesinde ara commit yap
5. İlerlemeyi kullanıcıya bildir

**ASLA "çok fazla hata var, rebrand'i geri alayım" deme.**

---

## REFERANS: REBRAND_MAP

Tam crate isim değişiklikleri için `/home/mamostehp/pezkuwi-sdk/REBRAND_PROGRESS.md` dosyasına bak.
Terminoloji kılavuzu için `.claude/TERMINOLOGY.md` dosyasına bak.

---

*Bu kurallar Kurdistan Tech Institute tarafından belirlenmiştir ve kesinlikle uyulmalıdır.*
