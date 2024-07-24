#!/usr/bin/env bash
set -Eeuo pipefail

pacman --noconfirm -Syu git curl
pacman --noconfirm -Syu pkgconf gcc glibc openssl libxml2
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
export PATH=$HOME/.cargo/bin:$PATH
git clone https://github.com/Orange-OpenSource/hurl.git
