#!/bin/bash
# SessionStart Hook - Otomatik Context Yükleme
# Bu script her yeni Claude oturumunda otomatik çalışır

set -e

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-/home/mamostehp/pezkuwi-sdk}"

echo "=============================================="
echo "PEZKUWI SDK - OTOMATIK CONTEXT YÜKLEME"
echo "=============================================="
echo ""

# CRITICAL_STATE - Tek kaynak dosya
if [ -f "$PROJECT_DIR/.claude/CRITICAL_STATE.md" ]; then
  echo "## KRİTİK DURUM ##"
  cat "$PROJECT_DIR/.claude/CRITICAL_STATE.md"
  echo ""
fi

echo "=============================================="
echo "CONTEXT YÜKLEME TAMAMLANDI"
echo "=============================================="

exit 0
