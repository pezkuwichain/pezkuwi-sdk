import os
import re

# Dosya Yolları ve Yapılacak İşlemler
# Format: (Dosya Yolu, [ (Aranacak_Regex, Yeni_Deger), (Alternatif_Basit_String, Yeni_Deger) ])
TASKS = [
    (
        "bizinikiwi/pezframe/identity/src/migration.rs",
        [
            (r"\[\s*u8\s*;\s*15\s*\]", "[u8; 18]"), # Regex (Esnek)
            ("[u8; 15]", "[u8; 18]"),               # Basit String (Yedek)
            ("[u8;15]", "[u8; 18]")                 # Basit String (Yedek 2)
        ]
    ),
    (
        "bizinikiwi/pezframe/revive/src/migrations/v1.rs",
        [
            (r"\[\s*u8\s*;\s*17\s*\]", "[u8; 20]"),
            ("[u8; 17]", "[u8; 20]"),
            ("[u8;17]", "[u8; 20]")
        ]
    ),
    (
        "bizinikiwi/pezframe/revive/src/migrations/v2.rs",
        [
            (r"\[\s*u8\s*;\s*17\s*\]", "[u8; 20]"),
            ("[u8; 17]", "[u8; 20]"),
            ("[u8;17]", "[u8; 20]")
        ]
    ),
    (
        "pezcumulus/teyrchains/pezpallets/ping/src/lib.rs",
        [
            (r"pezcumulus_pezpallet_xcm", "pezcumulus_pezpallet_xcm"),
            ("pezcumulus_pezpallet_xcm", "pezcumulus_pezpallet_xcm")
        ]
    )
]

def apply_fixes():
    root_dir = os.getcwd()
    print("--- Final Cerrahi Müdahale Başlıyor ---")
    
    total_fixed = 0

    for rel_path, replacements in TASKS:
        filepath = os.path.join(root_dir, rel_path)
        
        if not os.path.exists(filepath):
            print(f"[UYARI] Dosya bulunamadı: {rel_path}")
            continue

        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            file_modified = False
            
            # Her bir kuralı sırayla dene
            for pattern, replacement in replacements:
                # Regex mi String mi?
                if pattern.startswith("r") or "\\" in pattern or "[" in pattern:
                     # Regex denemesi
                     new_content, count = re.subn(pattern, replacement, content)
                     if count > 0:
                         content = new_content
                         file_modified = True
                         print(f"  -> [REGEX] {rel_path}: {replacement} ({count} adet)")
                
                # Basit String Replace (Regex kaçırırsa diye)
                if replacement not in content and pattern in content:
                    content = content.replace(pattern, replacement)
                    file_modified = True
                    print(f"  -> [STRING] {rel_path}: {replacement} (Basit değişim)")

            if file_modified:
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(content)
                total_fixed += 1
                print(f"  [KAYDEDİLDİ] {rel_path}")
            else:
                # Dosyada değişiklik olmadı, peki hedef string hala orada mı?
                # İlk kuralın hedef desenini kontrol edelim (basit string haliyle)
                check_pattern = replacements[0][0].replace(r"\[\s*u8\s*;\s*", "").replace(r"\s*\]", "") # Basitleştirme çabası
                if "15" in check_pattern and "[u8; 15]" in original_content:
                     print(f"  [HATA] {rel_path} içinde '[u8; 15]' var ama değiştirilemedi!")
                elif "17" in check_pattern and "[u8; 17]" in original_content:
                     print(f"  [HATA] {rel_path} içinde '[u8; 17]' var ama değiştirilemedi!")
                else:
                     print(f"  [BİLGİ] {rel_path}: Temiz görünüyor (Değişiklik gerekmedi).")

        except Exception as e:
            print(f"  [KRİTİK HATA] {rel_path}: {e}")

    print(f"\n--- İşlem Tamamlandı. {total_fixed} dosya güncellendi. ---")

if __name__ == "__main__":
    apply_fixes()
