#!/usr/bin/env bash

# Directory setup
SBF_PROGRAM_DIR=$PWD/integration_tests/tests/fixtures
FIXTURES_DIR=$PWD/tip-router-operator-cli/tests/fixtures
VRF_DIR=$FIXTURES_DIR/vrf
mkdir -p $VRF_DIR

# Program IDs from vrf_devnet_test.rs
VRF_COORDINATOR="BfwfooykCSdb1vgu6FcP75ncUgdcdt4ciUaeaSLzxM4D"
VRF_VERIFY="4qqRVYJAeBynm2yTydBkTJ9wVay3CrUfZ7gf9chtWS5Y"

# Create VRF keypairs
create_vrf_keypair() {
    if test ! -f "$1"
    then
        solana-keygen new --no-passphrase -s -o "$1"
    fi
}

echo "Creating VRF keypairs..."
create_vrf_keypair "$VRF_DIR/vrf_authority.json"
create_vrf_keypair "$VRF_DIR/subscription.json"

# Build VRF programs
echo "Building VRF programs..."
(cd mangekyou/kamui-program && cargo build-sbf --sbf-out-dir ../../target/deploy)

# Deploy VRF programs to test validator
echo "Deploying VRF programs..."
solana-test-validator \
    --bpf-program $VRF_COORDINATOR target/deploy/kamui_program.so \
    --bpf-program $VRF_VERIFY target/deploy/vrf_verify.so \
    --reset

# Initialize VRF subscription
echo "Initializing VRF subscription..."
RUST_LOG=info cargo run --bin mangekyou-cli -- \
    --keypair $VRF_DIR/vrf_authority.json \
    init-subscription \
    --min-balance 1000000 \
    --confirmations 1

echo "VRF test environment setup complete" 