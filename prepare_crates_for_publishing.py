#!/usr/bin/env python3
"""
Pezkuwi SDK - crates.io Publishing Preparation Script

Bu script tüm workspace crate'lerini tarar ve crates.io için gerekli
metadata'yı ekler/günceller. tomlkit kullanarak mevcut formatı korur.

Yapılanlar:
1. description yoksa crate adından oluşturur
2. documentation = "https://docs.rs/{crate_name}" eklenir
3. Eksik workspace inheritance alanları eklenir

Kullanım:
    python3 prepare_crates_for_publishing.py [--dry-run] [--report-only]
"""

import os
import re
import sys
from pathlib import Path
from typing import Dict, List, Tuple, Optional
import argparse

try:
    import tomlkit
    from tomlkit import document, table, inline_table
except ImportError:
    print("tomlkit gerekli: pip3 install --user tomlkit")
    sys.exit(1)

# Workspace root
WORKSPACE_ROOT = Path("/home/mamostehp/kurdistan-sdk")

# crates.io'da yayınlanmayacak pattern'ler (test, example, internal)
NO_PUBLISH_PATTERNS = [
    r".*-test$",
    r".*-tests$",
    r".*-testing$",
    r".*-mock$",
    r".*-mocks$",
    r".*-fuzzer$",
    r".*-fixtures$",
    r".*-bench$",
    r".*-benchmarks$",
    r"^test-.*",
    r"^testing-.*",
    r"^mock-.*",
    r".*-emulated-chain$",
    r".*-emulated$",
    r".*integration-tests.*",
    r"^adder.*",
    r"^halving-mega.*",
    r"^undying.*",
    r"^penpal.*",
    r"^minimal-template.*",
    r"^parachain-template.*",
    r"^teyrchain-template.*",
    r"^solochain-template.*",
]

# Description oluşturmak için prefix mapping
PREFIX_DESCRIPTIONS = {
    "pezsp-": "Pezkuwi SDK primitive: ",
    "pezsc-": "Pezkuwi SDK client component: ",
    "pezpallet-": "Pezkuwi SDK FRAME pallet: ",
    "pezframe-": "Pezkuwi SDK FRAME support: ",
    "pezcumulus-": "Pezkuwi SDK cumulus/teyrchain component: ",
    "pezkuwi-": "Pezkuwi SDK relay chain component: ",
    "bizinikiwi-": "Bizinikiwi utility: ",
    "xcm-": "Pezkuwi SDK XCM component: ",
    "staging-": "Pezkuwi SDK staging component: ",
    "pezstaging-": "Pezkuwi SDK staging component: ",
    "bp-": "Pezkuwi SDK bridge primitive: ",
    "pezpallet-bridge-": "Pezkuwi SDK bridge pallet: ",
    "snowbridge-": "Pezkuwi SDK Snowbridge component: ",
    "asset-hub-": "Pezkuwi SDK Asset Hub: ",
    "bridge-hub-": "Pezkuwi SDK Bridge Hub: ",
    "people-": "Pezkuwi SDK People chain: ",
    "coretime-": "Pezkuwi SDK Coretime chain: ",
    "collectives-": "Pezkuwi SDK Collectives chain: ",
    "glutton-": "Pezkuwi SDK Glutton chain: ",
}


def should_publish(crate_name: str, cargo_toml_path: Path) -> bool:
    """Crate'in crates.io'da yayınlanıp yayınlanmayacağını belirle."""
    # NO_PUBLISH_PATTERNS kontrol et
    for pattern in NO_PUBLISH_PATTERNS:
        if re.match(pattern, crate_name):
            return False

    # Path-based kontroller
    path_str = str(cargo_toml_path)
    if "/test/" in path_str or "/tests/" in path_str:
        return False
    if "/examples/" in path_str or "/example/" in path_str:
        return False
    if "/fuzzer/" in path_str:
        return False
    if "/integration-tests/" in path_str:
        return False
    if "/emulated/" in path_str:
        return False

    return True


def generate_description(crate_name: str) -> str:
    """Crate adından description oluştur."""
    # Prefix'e göre açıklama oluştur
    for prefix, desc_prefix in PREFIX_DESCRIPTIONS.items():
        if crate_name.startswith(prefix):
            # Prefix'i kaldır ve human-readable yap
            suffix = crate_name[len(prefix):]
            readable = suffix.replace("-", " ").replace("_", " ")
            return f"{desc_prefix}{readable}"

    # Default description
    readable = crate_name.replace("-", " ").replace("_", " ")
    return f"Pezkuwi SDK component: {readable}"


def parse_cargo_toml(path: Path) -> Optional[tomlkit.TOMLDocument]:
    """Cargo.toml dosyasını parse et (format koruyarak)."""
    try:
        with open(path, "r", encoding="utf-8") as f:
            return tomlkit.load(f)
    except Exception as e:
        print(f"  HATA: {path} parse edilemedi: {e}")
        return None


def write_cargo_toml(path: Path, data: tomlkit.TOMLDocument) -> bool:
    """Cargo.toml dosyasını yaz (format koruyarak)."""
    try:
        with open(path, "w", encoding="utf-8") as f:
            f.write(tomlkit.dumps(data))
        return True
    except Exception as e:
        print(f"  HATA: {path} yazılamadı: {e}")
        return False


def update_crate_metadata(cargo_path: Path, dry_run: bool = False) -> Tuple[str, bool, List[str]]:
    """
    Bir crate'in metadata'sını güncelle.

    Returns:
        (crate_name, was_modified, list_of_changes)
    """
    data = parse_cargo_toml(cargo_path)
    if data is None:
        return ("PARSE_ERROR", False, [f"Parse hatası: {cargo_path}"])

    # [package] section kontrolü
    if "package" not in data:
        return ("NO_PACKAGE", False, [f"[package] section yok: {cargo_path}"])

    package = data["package"]
    crate_name = package.get("name", "UNKNOWN")
    changes = []
    modified = False

    # Helper: workspace inline table oluştur
    def make_workspace_true():
        it = tomlkit.inline_table()
        it["workspace"] = True
        return it

    # 1. description - yoksa ekle
    if "description" not in package:
        desc = generate_description(crate_name)
        package["description"] = desc
        changes.append(f'description = "{desc}"')
        modified = True

    # 2. documentation - yoksa ekle
    if "documentation" not in package:
        doc_url = f"https://docs.rs/{crate_name}"
        package["documentation"] = doc_url
        changes.append(f'documentation = "{doc_url}"')
        modified = True

    # 3. repository.workspace = true - yoksa ekle
    if "repository" not in package:
        package["repository"] = make_workspace_true()
        changes.append("repository.workspace = true")
        modified = True

    # 4. homepage.workspace = true - yoksa ekle
    if "homepage" not in package:
        package["homepage"] = make_workspace_true()
        changes.append("homepage.workspace = true")
        modified = True

    # 5. authors.workspace = true - yoksa ekle
    if "authors" not in package:
        package["authors"] = make_workspace_true()
        changes.append("authors.workspace = true")
        modified = True

    # 6. edition.workspace = true - yoksa ekle (genellikle var)
    if "edition" not in package:
        package["edition"] = make_workspace_true()
        changes.append("edition.workspace = true")
        modified = True

    # 7. license - yoksa workspace'ten al
    if "license" not in package:
        package["license"] = make_workspace_true()
        changes.append("license.workspace = true")
        modified = True

    # 8. publish = false - test crate'leri için
    should_pub = should_publish(crate_name, cargo_path)
    current_publish = package.get("publish")
    if current_publish is None and not should_pub:
        package["publish"] = False
        changes.append("publish = false")
        modified = True

    if modified and not dry_run:
        write_cargo_toml(cargo_path, data)

    return (crate_name, modified, changes)


def find_all_cargo_tomls(root: Path) -> List[Path]:
    """Workspace'teki tüm Cargo.toml dosyalarını bul."""
    cargo_files = []
    # Hariç tutulacak dizinler
    exclude_dirs = {"target", "vendor", ".git"}

    for cargo_path in root.rglob("Cargo.toml"):
        # Root Cargo.toml'u atla (workspace tanımı)
        if cargo_path == root / "Cargo.toml":
            continue
        # Hariç tutulan dizinleri atla
        if any(part in exclude_dirs for part in cargo_path.parts):
            continue
        cargo_files.append(cargo_path)
    return sorted(cargo_files)


def main():
    parser = argparse.ArgumentParser(description="Pezkuwi SDK crates.io hazırlık script'i")
    parser.add_argument("--dry-run", action="store_true", help="Değişiklik yapma, sadece göster")
    parser.add_argument("--report-only", action="store_true", help="Sadece rapor oluştur")
    args = parser.parse_args()

    print("=" * 70)
    print("Pezkuwi SDK - crates.io Publishing Preparation")
    print("=" * 70)
    print()

    if args.dry_run:
        print("** DRY RUN MODE - Değişiklik yapılmayacak **\n")

    cargo_files = find_all_cargo_tomls(WORKSPACE_ROOT)
    print(f"Toplam {len(cargo_files)} Cargo.toml dosyası bulundu.\n")

    # İstatistikler
    stats = {
        "total": len(cargo_files),
        "modified": 0,
        "publish_true": 0,
        "publish_false": 0,
        "errors": 0,
    }

    results = []

    for cargo_path in cargo_files:
        rel_path = cargo_path.relative_to(WORKSPACE_ROOT)
        crate_name, modified, changes = update_crate_metadata(cargo_path, dry_run=args.dry_run or args.report_only)

        if crate_name in ["PARSE_ERROR", "NO_PACKAGE"]:
            stats["errors"] += 1
            results.append((str(rel_path), crate_name, changes))
            continue

        if modified:
            stats["modified"] += 1

        # Publish durumu tekrar hesapla
        should_pub = should_publish(crate_name, cargo_path)
        if should_pub:
            stats["publish_true"] += 1
        else:
            stats["publish_false"] += 1

        results.append((str(rel_path), crate_name, changes))

    # Rapor
    print("\n" + "=" * 70)
    print("RAPOR")
    print("=" * 70)

    # Değişiklik yapılanlar
    modified_crates = [(p, n, c) for p, n, c in results if c and n not in ["PARSE_ERROR", "NO_PACKAGE"]]
    if modified_crates:
        print(f"\n{len(modified_crates)} crate güncellendi:\n")
        for rel_path, crate_name, changes in modified_crates[:50]:  # İlk 50'yi göster
            print(f"  {crate_name}")
            for change in changes:
                print(f"    + {change}")
        if len(modified_crates) > 50:
            print(f"  ... ve {len(modified_crates) - 50} crate daha")

    # Hatalar
    errors = [(p, n, c) for p, n, c in results if n in ["PARSE_ERROR", "NO_PACKAGE"]]
    if errors:
        print(f"\n{len(errors)} HATA:\n")
        for rel_path, name, changes in errors:
            print(f"  {rel_path}: {changes[0] if changes else name}")

    # Özet
    print("\n" + "-" * 70)
    print("ÖZET")
    print("-" * 70)
    print(f"  Toplam Cargo.toml       : {stats['total']}")
    print(f"  Güncellenen             : {stats['modified']}")
    print(f"  Yayınlanacak (publish)  : {stats['publish_true']}")
    print(f"  Yayınlanmayacak         : {stats['publish_false']}")
    print(f"  Hatalar                 : {stats['errors']}")
    print()

    if args.dry_run or args.report_only:
        print("** Bu bir dry-run/report idi. Değişiklik uygulamak için:")
        print("   python3 prepare_crates_for_publishing.py")
    else:
        print("** Değişiklikler uygulandı! **")


if __name__ == "__main__":
    main()
