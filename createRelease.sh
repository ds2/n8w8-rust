#!/usr/bin/env bash

# i686-unknown-linux-gnu
export CARGO_BUILD_TARGET=${CARGO_BUILD_TARGET:-'x86_64-unknown-linux-gnu'}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/n8w8-rust}

if [ -f "$HOME/.cargo/env" ]; then
  source "$HOME/.cargo/env"
fi

while getopts ':orcb' OPTION; do
  case "$OPTION" in
  b)
    echo "Starting build into ${CARGO_TARGET_DIR} for machine ${CARGO_BUILD_TARGET}.."
    time cargo build -p n8w8d --release
    ;;
  c)
    echo "Cleaning target dir ${CARGO_TARGET_DIR}.."
    cargo clean
    ;;
  o)
    echo "Installing OS Updates.."
    dnf -y update
    echo "Installing build deps.."
    dnf -y install gcc openssl-devel pkg-config
    ;;
  r)
    echo "Installing Rust first.."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --profile minimal -y
    source "$HOME/.cargo/env"
    echo "Setting up compiler.."
    rustup target add ${CARGO_BUILD_TARGET}
    ;;
  \?) echo "Invalid param: $OPTARG" ;;
  *) echo "unknown parameter" ;;
  esac
done

if [ -f ${CARGO_TARGET_DIR}/${CARGO_BUILD_TARGET}/release/n8w8d ]; then
  echo "Copying final binary to output directory.."
  cp --update ${CARGO_TARGET_DIR}/${CARGO_BUILD_TARGET}/release/n8w8d /out/n8w8d-${CARGO_BUILD_TARGET}
fi
