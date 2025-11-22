# PDF Generation Guide

This guide explains how to convert WHITEPAPER.md and INVESTOR_DECK.md to professional PDFs.

## Prerequisites

You need to install pandoc and LaTeX support. Choose one of the methods below:

### Method 1: Using Pandoc (Recommended)

**On Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y pandoc texlive-xetex texlive-fonts-recommended texlive-fonts-extra
```

**On macOS:**
```bash
brew install pandoc
brew install basictex
```

**On Windows:**
1. Download pandoc from https://pandoc.org/installing.html
2. Download MiKTeX from https://miktex.org/download

### Method 2: Using Docker (No Local Installation)

```bash
docker run --rm -v $(pwd):/data pandoc/latex \
  WHITEPAPER.md -o WHITEPAPER.pdf \
  --pdf-engine=xelatex \
  --toc \
  --highlight-style=tango \
  -V geometry:margin=1in \
  -V linkcolor:blue \
  -V urlcolor:blue
```

## Generate PDFs

Once pandoc is installed, run these commands from the `/pezkuwi/docs` directory:

### Generate Whitepaper PDF

```bash
pandoc WHITEPAPER.md -o WHITEPAPER.pdf \
  --pdf-engine=xelatex \
  --toc \
  --toc-depth=3 \
  --number-sections \
  --highlight-style=tango \
  -V geometry:margin=1in \
  -V fontsize=11pt \
  -V documentclass=report \
  -V linkcolor:blue \
  -V urlcolor:blue \
  -V toccolor:blue \
  --metadata title="PezkuwiChain Whitepaper v3.0" \
  --metadata author="PezkuwiChain Team" \
  --metadata date="November 17, 2025"
```

**Expected Output:** `WHITEPAPER.pdf` (~60-80 pages)

### Generate Investor Deck PDF

```bash
pandoc INVESTOR_DECK.md -o INVESTOR_DECK.pdf \
  --pdf-engine=xelatex \
  --toc \
  --highlight-style=tango \
  -V geometry:margin=0.75in \
  -V fontsize=12pt \
  -V documentclass=article \
  -V linkcolor:blue \
  -V urlcolor:blue \
  --metadata title="PezkuwiChain Investor Deck" \
  --metadata author="PezkuwiChain Team" \
  --metadata date="November 17, 2025"
```

**Expected Output:** `INVESTOR_DECK.pdf` (~25-30 pages)

## Advanced Options

### Custom Styling (Professional Theme)

Create a file `custom-theme.yaml`:

```yaml
---
geometry: margin=1in
fontsize: 11pt
mainfont: Georgia
monofont: Courier New
linkcolor: navy
urlcolor: navy
toccolor: black
header-includes: |
  \usepackage{fancyhdr}
  \pagestyle{fancy}
  \fancyhead[L]{PezkuwiChain Whitepaper}
  \fancyhead[R]{\thepage}
  \fancyfoot[C]{Confidential - For Investor Use Only}
---
```

Then generate with custom theme:

```bash
pandoc WHITEPAPER.md -o WHITEPAPER.pdf \
  --pdf-engine=xelatex \
  --toc \
  --metadata-file=custom-theme.yaml
```

### Presentation Mode (Slides)

Generate slide deck instead of document:

```bash
pandoc INVESTOR_DECK.md -o INVESTOR_DECK_SLIDES.pdf \
  -t beamer \
  --pdf-engine=xelatex \
  -V theme:Madrid \
  -V colortheme:dolphin \
  --slide-level=2
```

## Troubleshooting

### Error: "xelatex not found"

Install complete LaTeX distribution:

**Ubuntu/Debian:**
```bash
sudo apt-get install texlive-full
```

**macOS:**
```bash
brew install mactex-no-gui
```

### Error: "Missing fonts"

Install additional fonts:

**Ubuntu/Debian:**
```bash
sudo apt-get install fonts-liberation fonts-dejavu
```

**macOS:**
Fonts are usually pre-installed. Use Font Book to verify.

### Large File Size

Compress PDF after generation:

```bash
gs -sDEVICE=pdfwrite -dCompatibilityLevel=1.4 -dPDFSETTINGS=/ebook \
   -dNOPAUSE -dQUIET -dBATCH \
   -sOutputFile=WHITEPAPER_compressed.pdf WHITEPAPER.pdf
```

## Alternative Tools (If Pandoc Unavailable)

### 1. Online Converters

- **Dillinger:** https://dillinger.io/ (Markdown editor with PDF export)
- **StackEdit:** https://stackedit.io/ (Online markdown editor)
- **Markdown to PDF:** https://www.markdowntopdf.com/

**Steps:**
1. Copy markdown content to online editor
2. Use "Export to PDF" feature
3. Download generated PDF

### 2. VS Code Extension

Install "Markdown PDF" extension in VS Code:

1. Open VS Code
2. Install extension: `yzane.markdown-pdf`
3. Open `WHITEPAPER.md`
4. Press `Ctrl+Shift+P` → Type "Markdown PDF: Export (pdf)"
5. PDF saved in same directory

### 3. GitHub Actions (Automated)

Create `.github/workflows/generate-pdf.yml`:

```yaml
name: Generate PDFs
on:
  push:
    paths:
      - 'pezkuwi/docs/WHITEPAPER.md'
      - 'pezkuwi/docs/INVESTOR_DECK.md'
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Pandoc
        run: |
          sudo apt-get update
          sudo apt-get install -y pandoc texlive-xetex
      - name: Generate PDFs
        run: |
          cd pezkuwi/docs
          pandoc WHITEPAPER.md -o WHITEPAPER.pdf --pdf-engine=xelatex --toc
          pandoc INVESTOR_DECK.md -o INVESTOR_DECK.pdf --pdf-engine=xelatex
      - name: Upload PDFs
        uses: actions/upload-artifact@v3
        with:
          name: pdfs
          path: pezkuwi/docs/*.pdf
```

## Verification

After generating PDFs, verify:

- ✅ All sections rendered correctly
- ✅ Table of contents links work
- ✅ Code blocks have syntax highlighting
- ✅ Tables are properly formatted
- ✅ Images/diagrams display (if any)
- ✅ Page numbers present
- ✅ Hyperlinks are clickable

## Distribution

Once PDFs are generated, you can:

1. **Host on Website:** Upload to https://pezkuwichain.io/docs/
2. **GitHub Release:** Attach to release tags
3. **Investor Portal:** Upload to secure investor portal
4. **Email:** Send to prospective investors

---

**Note:** If you cannot install software on your system, use Method 2 (Docker) or the online converter options. These require no local installation.
