#!/usr/bin/env bash
set -Eeuo pipefail

RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo build
target/debug/hurl integration/hurl/tests_ok/http_version_3_option.hurl
