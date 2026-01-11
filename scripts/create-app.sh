#!/bin/sh
# Cancer Create App - One-liner project creation
# Usage: curl -fsSL https://raw.githubusercontent.com/USER/cancer/main/scripts/create-app.sh | sh -s -- my-app
#
# This script downloads the cancer CLI to a temp directory, creates your project,
# and cleans up. No permanent installation required.

set -e

# Configuration
REPO="albertogferrario/cancer"
BINARY_NAME="cancer"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m'

info() { printf "${CYAN}>${NC} %s\n" "$1"; }
success() { printf "${GREEN}✓${NC} %s\n" "$1"; }
error() { printf "${RED}✗${NC} %s\n" "$1"; exit 1; }

# Check if project name provided
PROJECT_NAME="$1"
if [ -z "$PROJECT_NAME" ]; then
    echo ""
    printf "${CYAN}Cancer${NC} - Create a new project\n"
    echo ""
    echo "Usage:"
    echo "  curl -fsSL https://raw.githubusercontent.com/${REPO}/main/scripts/create-app.sh | sh -s -- <project-name>"
    echo ""
    echo "Example:"
    echo "  curl -fsSL https://raw.githubusercontent.com/${REPO}/main/scripts/create-app.sh | sh -s -- my-app"
    echo ""
    exit 1
fi

# Detect platform
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux) OS="linux" ;;
        Darwin) OS="darwin" ;;
        MINGW*|MSYS*|CYGWIN*) OS="windows" ;;
        *) error "Unsupported OS: $OS" ;;
    esac

    case "$ARCH" in
        x86_64|amd64) ARCH="x86_64" ;;
        arm64|aarch64) ARCH="aarch64" ;;
        *) error "Unsupported architecture: $ARCH" ;;
    esac

    # Map to Rust target triple
    case "${OS}-${ARCH}" in
        linux-x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
        linux-aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
        darwin-x86_64) TARGET="x86_64-apple-darwin" ;;
        darwin-aarch64) TARGET="aarch64-apple-darwin" ;;
        windows-x86_64) TARGET="x86_64-pc-windows-msvc" ;;
        *) error "Unsupported platform: ${OS}-${ARCH}" ;;
    esac
}

# Get latest version
get_latest_version() {
    VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')
    if [ -z "$VERSION" ]; then
        error "Failed to get latest version. Check your internet connection."
    fi
}

main() {
    echo ""
    printf "${CYAN}Creating Cancer project:${NC} %s\n" "$PROJECT_NAME"
    echo ""

    # Check if directory exists
    if [ -d "$PROJECT_NAME" ]; then
        error "Directory '$PROJECT_NAME' already exists"
    fi

    detect_platform
    get_latest_version

    info "Downloading Cancer CLI ($VERSION)..."

    # Create temp directory
    TMP_DIR=$(mktemp -d)
    trap "rm -rf $TMP_DIR" EXIT

    # Download
    if [ "$OS" = "windows" ]; then
        ARCHIVE="cancer-${VERSION}-${TARGET}.zip"
    else
        ARCHIVE="cancer-${VERSION}-${TARGET}.tar.gz"
    fi

    DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARCHIVE}"

    cd "$TMP_DIR"
    curl -fsSL "$DOWNLOAD_URL" -o "$ARCHIVE" || error "Failed to download Cancer CLI"

    # Extract
    if [ "$OS" = "windows" ]; then
        unzip -q "$ARCHIVE"
    else
        tar -xzf "$ARCHIVE"
    fi

    chmod +x "$BINARY_NAME" 2>/dev/null || true

    success "Downloaded Cancer CLI"

    # Create project
    info "Generating project structure..."

    cd - > /dev/null
    "$TMP_DIR/$BINARY_NAME" new "$PROJECT_NAME" --no-interaction --no-git

    # Initialize git
    info "Initializing git repository..."
    cd "$PROJECT_NAME"
    git init -q
    git add .
    git commit -q -m "Initial commit from Cancer"
    cd - > /dev/null

    success "Project created successfully!"

    echo ""
    echo "Next steps:"
    printf "  ${CYAN}cd %s${NC}\n" "$PROJECT_NAME"
    printf "  ${CYAN}cd frontend && npm install && cd ..${NC}\n"
    printf "  ${CYAN}cargo run -- migrate${NC}\n"
    printf "  ${CYAN}cargo run -- serve${NC}\n"
    echo ""
    printf "Or install Cancer CLI globally:\n"
    printf "  ${CYAN}cargo install cancer-cli${NC}\n"
    echo ""
}

main "$@"
