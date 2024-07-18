#!/usr/bin/env bash
set -Eeuo pipefail

RUSTFLAGS="-A dead_code" cargo build --release
target/release/hurl --verbose integration/hurl/tests_ok/http_version_3_option.hurl
