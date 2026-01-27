#!/bin/bash
set -e

VERSION="0.1.0"
ARCH="amd64"
PKG_NAME="alacritty-gtk"
DIR_NAME="${PKG_NAME}_${VERSION}_${ARCH}"

# Prepare directory
mkdir -p build_deb/${DIR_NAME}/DEBIAN
mkdir -p build_deb/${DIR_NAME}/usr/bin
mkdir -p build_deb/${DIR_NAME}/usr/share/applications

# Copy Binary
cp target/release/alacritty_gtk build_deb/${DIR_NAME}/usr/bin/

# Create Control File
cat <<EOF > build_deb/${DIR_NAME}/DEBIAN/control
Package: ${PKG_NAME}
Version: ${VERSION}
Architecture: ${ARCH}
Maintainer: Your Name <you@example.com>
Depends: libgtk-4-1, libgl1, libc6
Description: Alacritty terminal with native GTK integration
 A GPU-accelerated terminal emulator with GTK capabilities.
EOF

# Create Desktop Entry
cat <<EOF > build_deb/${DIR_NAME}/usr/share/applications/${PKG_NAME}.desktop
[Desktop Entry]
Type=Application
TryExec=alacritty_gtk
Exec=alacritty_gtk
Icon=alacritty
Terminal=false
Categories=System;TerminalEmulator;
Name=Alacritty GTK
GenericName=Terminal
Comment=A fast, cross-platform, OpenGL terminal emulator
StartupNotify=true
EOF

# Build Deb
dpkg-deb --build build_deb/${DIR_NAME}

# Move to output
mkdir -p /output
cp build_deb/${DIR_NAME}.deb /output/
echo "Package created at /output/${DIR_NAME}.deb"
