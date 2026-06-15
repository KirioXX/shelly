#!/usr/bin/env bash
set -euo pipefail

REPO="KirioXX/shelly"
API_URL="https://api.github.com/repos/${REPO}/releases/latest"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

say() {
	echo -e "${BLUE}→${NC} $1"
}

warn() {
	echo -e "${YELLOW}⚠${NC} $1"
}

error() {
	echo -e "${RED}✗${NC} $1" >&2
}

success() {
	echo -e "${GREEN}✓${NC} $1"
}

detect_os() {
	case "$(uname -s)" in
	Linux*) OS="ubuntu-latest" ;;
	Darwin*) OS="macos-latest" ;;
	CYGWIN* | MINGW* | MSYS*) OS="windows-latest" ;;
	*)
		error "Unsupported OS: $(uname -s)"
		exit 1
		;;
	esac
	say "Detected OS: ${OS}"
}

get_latest_release() {
	say "Fetching latest release info..."

	if ! command -v curl >/dev/null 2>&1; then
		error "curl is required but not installed."
		exit 1
	fi

	TAG=$(curl -fsSL "${API_URL}" | grep -o '"tag_name": "[^"]*"' | head -n 1 | sed 's/.*"tag_name": "\(.*\)".*/\1/')

	if [ -z "$TAG" ]; then
		error "Could not determine latest release tag."
		exit 1
	fi

	say "Latest release: ${TAG}"
}

find_asset_url() {
	say "Finding asset for ${OS}..."

	ASSETS=$(curl -fsSL "${API_URL}")

	if [ "$OS" = "windows-latest" ]; then
		EXT="zip"
		BINARY="shelly.exe"
	else
		EXT="tar.gz"
		BINARY="shelly"
	fi

	# Find the asset URL matching our OS
	PATTERN="shelly-${OS}-${TAG}.${EXT}"
	URL=$(echo "$ASSETS" | grep -o '"browser_download_url": "[^"]*"' | grep "${PATTERN}" | head -n 1 | sed 's/.*"browser_download_url": "\(.*\)".*/\1/')

	if [ -z "$URL" ]; then
		# Fallback: try without the tag suffix (some releases might differ)
		PATTERN="shelly-${OS}"
		URL=$(echo "$ASSETS" | grep -o '"browser_download_url": "[^"]*"' | grep "${PATTERN}" | head -n 1 | sed 's/.*"browser_download_url": "\(.*\)".*/\1/')
	fi

	if [ -z "$URL" ]; then
		error "Could not find a release asset for ${OS}."
		echo "  Available assets:"
		echo "$ASSETS" | grep -o '"name": "[^"]*"' | grep "shelly-" | sed 's/.*"name": "\(.*\)".*/  - \1/' || true
		exit 1
	fi

	say "Asset URL: ${URL}"
}

download() {
	TMPDIR=$(mktemp -d)
	trap 'rm -rf "$TMPDIR"' EXIT

	if [ "$OS" = "windows-latest" ]; then
		ARCHIVE="${TMPDIR}/shelly.zip"
	else
		ARCHIVE="${TMPDIR}/shelly.tar.gz"
	fi

	say "Downloading to ${ARCHIVE}..."
	curl -fsSL -o "${ARCHIVE}" "$URL"
	success "Downloaded $(du -h "${ARCHIVE}" | cut -f1)"
}

extract() {
	say "Extracting archive..."

	if [ "$OS" = "windows-latest" ]; then
		unzip -q "${ARCHIVE}" -d "$TMPDIR"
		BINARY_PATH="${TMPDIR}/${BINARY}"
	else
		tar -xzf "${ARCHIVE}" -C "$TMPDIR"
		BINARY_PATH="${TMPDIR}/${BINARY}"
	fi

	if [ ! -f "$BINARY_PATH" ]; then
		error "Binary not found after extraction."
		echo "  Contents of ${TMPDIR}:"
		ls -la "$TMPDIR" || true
		exit 1
	fi

	success "Extracted ${BINARY}"
}

install_binary() {
	if [ -n "${INSTALL_DIR:-}" ]; then
		INSTALL_DIR="${INSTALL_DIR}"
	elif [ -d "$HOME/.local/bin" ]; then
		INSTALL_DIR="$HOME/.local/bin"
	else
		INSTALL_DIR="$HOME/.local/bin"
		mkdir -p "$INSTALL_DIR"
	fi

	DEST="${INSTALL_DIR}/${BINARY}"

	say "Installing to ${DEST}..."

	if [ -f "$DEST" ]; then
		warn "Existing binary found at ${DEST}"
		mv "$DEST" "${DEST}.backup"
	fi

	cp "$BINARY_PATH" "$DEST"
	chmod +x "$DEST"

	success "Installed shelly ${TAG} to ${DEST}"
}

check_path() {
	case ":${PATH}:" in
	*:"${INSTALL_DIR}":*)
		success "${INSTALL_DIR} is already in your PATH"
		;;
	*)
		warn "${INSTALL_DIR} is not in your PATH."
		echo ""
		echo "  Add this to your shell config (.bashrc, .zshrc, etc.):"
		echo "    export PATH=\"${INSTALL_DIR}:\$PATH\""
		echo ""
		;;
	esac
}

verify_install() {
	if command -v "$DEST" >/dev/null 2>&1; then
		VERSION=$($DEST --version 2>/dev/null || echo "unknown")
		success "Installation verified: ${VERSION}"
	else
		warn "Installation succeeded but '$DEST' is not in your current PATH."
	fi
}

main() {
	echo ""
	echo "  🐚 Shelly Installer"
	echo ""

	detect_os
	get_latest_release
	find_asset_url
	download
	extract
	install_binary
	check_path
	verify_install

	echo ""
	success "Done! Run 'shelly setup' to configure."
	echo ""
}

main "$@"
