#!/usr/bin/env bash

RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo build
target/debug/hurl integration/hurl/tests_ok/http_version_3_option.hurl
