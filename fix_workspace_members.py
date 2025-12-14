import os
import re

def fix_rocksdb_version():
    print("🪨  kvdb-rocksdb versiyonu (Workspace dahil) 0.21.0'a yükseltiliyor...")
    modified_files = 0
    
    # HEDEF: Hem `kvdb-rocksdb = "0.20.1"` hem de `kvdb-rocksdb = { version = "0.20.1" }` formatlarını yakalamak.
    
    for root, dirs, files in os.walk("."):
        if "Cargo.toml" in files:
            filepath = os.path.join(root, "Cargo.toml")
            
            try:
                with open(filepath, "r", encoding="utf-8") as f:
                    lines = f.readlines()
                
                new_lines = []
                file_changed = False
                
                for line in lines:
                    # Satırda kvdb-rocksdb geçiyor mu?
                    if "kvdb-rocksdb" in line:
                        # Ve versiyon olarak 0.20 geçiyor mu?
                        if "0.20" in line:
                            # 0.20.x desenini bul ve 0.21.0 ile değiştir
                            # Bu regex 0.20.1, 0.20, 0.20.99 gibi varyasyonları yakalar
                            line = re.sub(r'0\.20(\.\d+)?', '0.21.0', line)
                            file_changed = True
                    
                    new_lines.append(line)

                if file_changed:
                    with open(filepath, "w", encoding="utf-8") as f:
                        f.writelines(new_lines)
                    print(f"  ✅ Güncellendi: {filepath}")
                    modified_files += 1
                    
            except Exception as e:
                print(f"  ❌ Hata: {filepath} - {e}")

    print(f"\nToplam {modified_files} dosyada versiyon güncellendi.")

if __name__ == "__main__":
    fix_rocksdb_version_v2()