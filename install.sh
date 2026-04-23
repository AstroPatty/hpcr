#!/bin/sh
set -eu

REPO="AstroPatty/hpcr"
BINARY="hpcr"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

die() { echo "error: $*" >&2; exit 1; }

# Verify platform
OS=$(uname -s)
ARCH=$(uname -m)
[ "$OS" = "Linux" ]  || die "unsupported OS '$OS'; hpcr supports Linux only"
[ "$ARCH" = "x86_64" ] || die "unsupported architecture '$ARCH'; hpcr supports x86_64 only"

# Resolve version
if [ -z "${HPCR_VERSION:-}" ]; then
    HPCR_VERSION=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
        | grep '"tag_name"' \
        | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')
    [ -n "$HPCR_VERSION" ] || die "could not determine latest release; set HPCR_VERSION to install a specific version"
fi

TARBALL="hpcr-x86_64-unknown-linux-gnu.tar.gz"
URL="https://github.com/$REPO/releases/download/$HPCR_VERSION/$TARBALL"

echo "Installing $BINARY $HPCR_VERSION to $INSTALL_DIR..."

mkdir -p "$INSTALL_DIR"
curl -fsSL "$URL" | tar xz -C "$INSTALL_DIR" "$BINARY"
chmod +x "$INSTALL_DIR/$BINARY"

echo "Installed $INSTALL_DIR/$BINARY"

# Warn if install dir is not on PATH
case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *) echo "warning: $INSTALL_DIR is not on your PATH" ;;
esac

"$INSTALL_DIR/$BINARY" setup
