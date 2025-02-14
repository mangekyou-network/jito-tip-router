#!/bin/bash

# Default values
RPC_URL="https://api.devnet.solana.com"
KEYPAIR_PATH="~/.config/solana/id.json"
VRF_COORDINATOR="BfwfooykCSdb1vgu6FcP75ncUgdcdt4ciUaeaSLzxM4D"
VRF_VERIFY="4qqRVYJAeBynm2yTydBkTJ9wVay3CrUfZ7gf9chtWS5Y"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --rpc-url)
            RPC_URL="$2"
            shift 2
            ;;
        --keypair)
            KEYPAIR_PATH="$2"
            shift 2
            ;;
        --subscription)
            SUBSCRIPTION="$2"
            shift 2
            ;;
        --coordinator)
            VRF_COORDINATOR="$2"
            shift 2
            ;;
        --verify)
            VRF_VERIFY="$2"
            shift 2
            ;;
        *)
            echo "Unknown argument: $1"
            exit 1
            ;;
    esac
done

# Check required arguments
if [ -z "$SUBSCRIPTION" ]; then
    echo "Error: --subscription is required"
    exit 1
fi

# Run the VRF prover node
RUST_LOG=info cargo run --bin tip-router-operator-cli -- \
    --keypair-path "$KEYPAIR_PATH" \
    --rpc-url "$RPC_URL" \
    --vrf-enabled \
    --vrf-coordinator-program "$VRF_COORDINATOR" \
    --vrf-verify-program "$VRF_VERIFY" \
    run \
    --vrf-subscription "$SUBSCRIPTION" 