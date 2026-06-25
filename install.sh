#!/usr/bin/env bash
set -euo pipefail

REPO="financia0x00001/claude-code-cleaner"
BIN="claude-code-cleaner"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*" >&2; exit 1; }

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)   echo "unknown-linux-gnu" ;;
        Darwin*)  echo "apple-darwin" ;;
        MINGW*|MSYS*|CYGWIN*|WSL*) echo "pc-windows-msvc" ;;
        *)        error "Unsupported OS: $(uname -s). Supported: Linux, macOS, Windows (Git Bash/WSL)." ;;
    esac
}

# Detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)  echo "x86_64" ;;
        arm64|aarch64) echo "aarch64" ;;
        *)             error "Unsupported architecture: $(uname -m). Only x86_64 and aarch64 are supported." ;;
    esac
}

# Get latest release tag
get_latest_version() {
    if command -v curl &>/dev/null; then
        curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/'
    elif command -v wget &>/dev/null; then
        wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/'
    else
        error "Neither curl nor wget found. Please install one of them."
    fi
}

main() {
    local version="${1:-}"
    local os arch target

    os="$(detect_os)"
    arch="$(detect_arch)"
    target="${arch}-${os}"

    info "Detected platform: ${target}"

    if [ -z "$version" ]; then
        info "Fetching latest release..."
        version="$(get_latest_version)"
        if [ -z "$version" ]; then
            error "Could not determine latest version. Specify a version: $0 v0.1.0"
        fi
    fi

    info "Installing ${BIN} ${version}..."

    local archive="${BIN}-${version}-${target}.zip"
    local url="https://github.com/${REPO}/releases/download/${version}/${archive}"

    local tmpdir
    tmpdir="$(mktemp -d)"
    trap 'rm -rf "$tmpdir"' EXIT

    info "Downloading ${url}..."
    if command -v curl &>/dev/null; then
        curl -fSL "$url" -o "${tmpdir}/${archive}" || error "Download failed. Check that version ${version} exists and has a binary for ${target}."
    else
        wget -q "$url" -O "${tmpdir}/${archive}" || error "Download failed. Check that version ${version} exists and has a binary for ${target}."
    fi

    info "Extracting..."
    if [ "$os" = "pc-windows-msvc" ]; then
        # Windows: use unzip (available in Git Bash)
        if command -v unzip &>/dev/null; then
            unzip -q "${tmpdir}/${archive}" -d "$tmpdir"
        else
            error "'unzip' not found. Please install it or use 7z/PowerShell to extract manually."
        fi
    else
        # Linux/macOS: use tar
        tar xzf "${tmpdir}/${archive%.zip}.tar.gz" -C "$tmpdir" 2>/dev/null || \
        tar xzf "${tmpdir}/${archive}" -C "$tmpdir" 2>/dev/null || \
        error "Failed to extract archive. The release may not have a ${target} package."
    fi

    # Determine install directory based on platform
    local install_dir
    if [ "$os" = "pc-windows-msvc" ]; then
        # Windows: install to %LOCALAPPDATA%\claude-code-cleaner\bin\
        install_dir="${LOCALAPPDATA:-$HOME/.local/share}/claude-code-cleaner/bin"
    else
        install_dir="${INSTALL_DIR:-/usr/local/bin}"
    fi

    info "Installing to ${install_dir}/${BIN}..."

    if [ "$os" = "pc-windows-msvc" ]; then
        mkdir -p "$install_dir"
        # Find the extracted binary (it's inside a staging folder)
        local binary
        binary="$(find "$tmpdir" -name "${BIN}.exe" -type f | head -n1)"
        if [ -z "$binary" ]; then
            error "Binary not found in archive. Expected a file named ${BIN}.exe"
        fi
        cp "$binary" "${install_dir}/${BIN}.exe"
        info "Successfully installed ${BIN} ${version} to ${install_dir}/${BIN}.exe"
        echo ""
        echo "  Run it with:  ${install_dir}/${BIN}.exe"
        echo ""
        echo "  Tip: Add to PATH by appending to your PowerShell profile:"
        echo "    \$env:PATH += ';${install_dir}'"
        echo "  Or add permanently via System Properties > Environment Variables."
    else
        if [ -w "$install_dir" ]; then
            cp "${tmpdir}/${BIN}-${version}-${target}/${BIN}" "${install_dir}/${BIN}"
            chmod +x "${install_dir}/${BIN}"
        else
            warn "Need sudo to install to ${install_dir}"
            sudo cp "${tmpdir}/${BIN}-${version}-${target}/${BIN}" "${install_dir}/${BIN}"
            sudo chmod +x "${install_dir}/${BIN}"
        fi
        info "Successfully installed ${BIN} ${version} to ${install_dir}/${BIN}"
        echo ""
        echo "  Run it with:  ${BIN}"
        echo ""
        echo "  Or specify a custom install directory:"
        echo "    INSTALL_DIR=~/.local/bin bash install.sh"
        echo ""
    fi
}

main "$@"
