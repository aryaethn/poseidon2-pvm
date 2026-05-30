#!/usr/bin/env bash
# On-chain ref_time measurement scaffold (Milestone 2). Fill in node + tooling specifics.
set -euo pipefail

NODE_RPC="${NODE_RPC:-ws://127.0.0.1:9944}"   # ink-node default; or a Paseo Asset Hub endpoint
CONTRACT_DIR="$(dirname "$0")/../crates/merkle-contract"

echo "[1/4] Build + link the PVM contract"
echo "    cd $CONTRACT_DIR && cargo build --release   # then link with polkavm-linker per template"

echo "[2/4] Deploy to $NODE_RPC"
echo "    TODO: deploy via polkadot-js / cast (Asset Hub revive RPC); capture the address"

echo "[3/4] Call entrypoints and read metered weight"
echo "    TODO: call insertLeaf / getRoot; extract ref_time + proof_size from the tx result"

echo "[4/4] Convert to block-budget % and write report/onchain-bench.md"
echo "    block_budget_ps=2060000000000   # ~2.06e12; pct = 100 * ref_time / block_budget_ps"

echo "Not yet implemented — Milestone 2."
