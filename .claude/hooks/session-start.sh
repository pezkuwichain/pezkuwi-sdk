#!/bin/bash
# SessionStart Hook - Otomatik Context Yükleme
# Bu script her yeni Claude oturumunda otomatik çalışır

set -e

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-/home/mamostehp/pezkuwi-sdk}"

echo "=============================================="
echo "PEZKUWI SDK - OTOMATIK CONTEXT YÜKLEME"
echo "=============================================="
echo ""

# 1. PROJECT STATE - Kritik bilgiler
if [ -f "$PROJECT_DIR/.claude/PROJECT_STATE.md" ]; then
  echo "## PROJECT STATE ##"
  cat "$PROJECT_DIR/.claude/PROJECT_STATE.md"
  echo ""
fi

# 2. SESSION LOG - Son oturum özeti
if [ -f "$PROJECT_DIR/.claude/SESSION_LOG.md" ]; then
  echo "## SON OTURUM ÖZETİ ##"
  cat "$PROJECT_DIR/.claude/SESSION_LOG.md"
  echo ""
fi

# 3. MAINNET ROADMAP - İlerleme durumu (sadece özet)
if [ -f "$PROJECT_DIR/.claude/MAINNET_ROADMAP.md" ]; then
  echo "## MAINNET İLERLEME (ÖZET) ##"
  grep -A 10 "## İLERLEME TAKİBİ" "$PROJECT_DIR/.claude/MAINNET_ROADMAP.md" 2>/dev/null || true
  echo ""
fi

echo "=============================================="
echo "CONTEXT YÜKLEME TAMAMLANDI"
echo "Detaylı bilgi için: .claude/PROJECT_STATE.md"
echo "=============================================="

exit 0
