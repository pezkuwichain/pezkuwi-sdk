import os
import re

# Taranacak Ana Klasörler (Buradaki tüm .rs dosyaları taranacak)
SCAN_DIRS = [
    "bizinikiwi/pezframe/identity",
    "bizinikiwi/pezframe/revive"
]

# Değişim Kuralları (Regex -> Yeni String)
# Regex'ler boşluk toleranslıdır: [u8;  15] veya [u8;15] yakalar.
REPLACEMENTS = [
    (r"\[\s*u8\s*;\s*15\s*\]", "[u8; 18]"),
    (r"\[\s*u8\s*;\s*17\s*\]", "[u8; 20]")
]

# Ekstra Tekil Dosya Düzeltmeleri (Import hataları vb.)
SINGLE_FILE_FIXES = [
    (
        "pezcumulus/teyrchains/pezpallets/ping/src/lib.rs",
        r"pezcumulus_pezpallet_xcm",
        "pezcumulus_pezpallet_xcm"
    )
]

def apply_fixes():
    root_dir = os.getcwd()
    print("--- Geniş Kapsamlı Tarama ve Düzeltme Başlıyor ---")
    
    total_fixed = 0

    # 1. Klasör Taraması (Array Boyutları İçin)
    for scan_dir in SCAN_DIRS:
        abs_scan_dir = os.path.join(root_dir, scan_dir)
        if not os.path.exists(abs_scan_dir):
            print(f"[UYARI] Klasör bulunamadı: {scan_dir}")
            continue

        print(f"Taranıyor: {scan_dir} ...")
        
        for dirpath, _, filenames in os.walk(abs_scan_dir):
            for filename in filenames:
                if not filename.endswith(".rs"):
                    continue
                
                filepath = os.path.join(dirpath, filename)
                try:
                    with open(filepath, 'r', encoding='utf-8') as f:
                        content = f.read()
                    
                    original_content = content
                    modified = False
                    
                    for pattern, replacement in REPLACEMENTS:
                        # Regex ile değiştir (subn kaç tane değiştiğini sayar)
                        new_content, count = re.subn(pattern, replacement, content)
                        if count > 0:
                            content = new_content
                            modified = True
                            print(f"  -> [DÜZELTİLDİ] {filename}: {replacement} yapıldı ({count} adet)")
                    
                    if modified:
                        with open(filepath, 'w', encoding='utf-8') as f:
                            f.write(content)
                        total_fixed += 1
                        
                except Exception as e:
                    print(f"  [HATA] {filename} okunurken hata: {e}")

    # 2. Tekil Dosya Düzeltmeleri
    print("\nTekil Dosyalar Kontrol Ediliyor...")
    for rel_path, pattern, replacement in SINGLE_FILE_FIXES:
        filepath = os.path.join(root_dir, rel_path)
        if os.path.exists(filepath):
            try:
                with open(filepath, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                new_content, count = re.subn(pattern, replacement, content)
                if count > 0:
                    with open(filepath, 'w', encoding='utf-8') as f:
                        f.write(new_content)
                    print(f"  -> [DÜZELTİLDİ] {rel_path} ({count} adet)")
                    total_fixed += 1
                elif replacement in content:
                    print(f"  -> [ATLANDI] {rel_path} (Zaten düzgün)")
            except Exception as e:
                 print(f"  [HATA] {rel_path}: {e}")
        else:
             print(f"  [UYARI] Dosya yok: {rel_path}")

    print(f"\n--- İşlem Tamamlandı. Toplam {total_fixed} dosya güncellendi. ---")

if __name__ == "__main__":
    apply_fixes()
