#!/usr/bin/env bash
# 本地开发安装 — 在仓库目录内编译 tz
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
# shellcheck source=scripts/lib/tz-common.sh
source "${ROOT}/scripts/lib/tz-common.sh"

echo ">> 本地编译 tz..."
(cd "${ROOT}/tz" && CARGO_TARGET_DIR="${ROOT}/tz/target" cargo build --release)

tz_install_binary "${ROOT}/tz/target/release/tz"
tz_install_shell
tz_print_done
