#!/usr/bin/env bash
# 一键安装 tz — 从 GitHub 下载预编译包或本地编译
#
#   curl -fsSL https://raw.githubusercontent.com/soul667/proxy_server/main/scripts/install.sh | bash
#
# 环境变量:
#   TZ_REPO=soul667/proxy_server   GitHub 仓库
#   TZ_TAG=tz-latest               Release 标签
#   TZ_NO_SHELL=1                  跳过 shell 配置
set -euo pipefail

TZ_REPO="${TZ_REPO:-soul667/proxy_server}"
TZ_TAG="${TZ_TAG:-tz-latest}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" && pwd)"

# curl | bash 时 BASH_SOURCE 可能为空，使用固定 raw 路径
if [[ -f "${SCRIPT_DIR}/lib/tz-common.sh" ]]; then
  # shellcheck source=scripts/lib/tz-common.sh
  source "${SCRIPT_DIR}/lib/tz-common.sh"
else
  TZ_TMP="$(mktemp -d)"
  trap 'rm -rf "${TZ_TMP}"' EXIT
  curl -fsSL "https://raw.githubusercontent.com/${TZ_REPO}/main/scripts/lib/tz-common.sh" \
    -o "${TZ_TMP}/tz-common.sh"
  # shellcheck source=/dev/null
  source "${TZ_TMP}/tz-common.sh"
fi

echo ">> 安装 tz (proxy switcher)..."

ARCH_TRIPLE="$(tz_detect_arch)"
DOWNLOAD_URL="https://github.com/${TZ_REPO}/releases/download/${TZ_TAG}/tz-${ARCH_TRIPLE}"
TMP_BIN="$(mktemp)"
trap 'rm -f "${TMP_BIN}"' EXIT

installed=0

if curl -fsSL --retry 3 --retry-delay 2 "${DOWNLOAD_URL}" -o "${TMP_BIN}" 2>/dev/null; then
  if [[ -s "${TMP_BIN}" ]] && head -c 4 "${TMP_BIN}" | grep -q $'^\x7fELF'; then
    echo ">> 已从 GitHub Release 下载 (${ARCH_TRIPLE})"
    tz_install_binary "${TMP_BIN}"
    installed=1
  fi
fi

if [[ "${installed}" -eq 0 ]]; then
  echo ">> Release 不可用，尝试从源码编译..."
  if ! command -v cargo >/dev/null 2>&1; then
    echo ""
    echo "错误: 未找到预编译包，且本机没有 cargo/Rust。"
    echo "  1) 安装 Rust: curl -fsSL https://sh.rustup.rs | sh"
    echo "  2) 或稍后再试（等 GitHub Actions 构建完成）"
    echo "  3) 手动: git clone https://github.com/${TZ_REPO}.git && ./scripts/install-tz.sh"
    exit 1
  fi

  BUILD_DIR="$(mktemp -d)"
  trap 'rm -rf "${BUILD_DIR}" "${TMP_BIN}"' EXIT
  git clone --depth 1 "https://github.com/${TZ_REPO}.git" "${BUILD_DIR}/repo"
  (cd "${BUILD_DIR}/repo/tz" && cargo build --release)
  tz_install_binary "${BUILD_DIR}/repo/tz/target/release/tz"
fi

if [[ "${TZ_NO_SHELL:-0}" != "1" ]]; then
  tz_install_shell
fi

tz_print_done
