#!/usr/bin/env bash

TGT_BIN_DIR="${BUILD_PACKAGE_ROOT:-/tmp/delme}/usr/local/bin/"
SRC_BIN_DIR="${CARGO_TARGET_DIR}/${CARGO_BUILD_TARGET}/${RUST_PROFILE}"

mkdir -p "${TGT_BIN_DIR}"

cp "${SRC_BIN_DIR}/n8w8-k8s" "${TGT_BIN_DIR}"
cp "${SRC_BIN_DIR}/n8w8-http-check" "${TGT_BIN_DIR}"
cp "${SRC_BIN_DIR}/n8w8-simple-val" "${TGT_BIN_DIR}"
