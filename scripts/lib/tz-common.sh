#!/usr/bin/env bash
# tz 安装公共函数 — 被 install.sh / install-tz.sh 引用
set -euo pipefail

TZ_BIN_DIR="${TZ_BIN_DIR:-${HOME}/.local/bin}"
TZ_CONFIG_DIR="${TZ_CONFIG_DIR:-${HOME}/.config/tz}"
TZ_MARKER="# >>> tz proxy >>>"
TZ_MARKER_END="# <<< tz proxy <<<"
TZ_PATH_MARKER="# >>> tz path >>>"
TZ_PATH_MARKER_END="# <<< tz path <<<"

tz_install_shell() {
  local snippet="${TZ_MARKER}
tz() {
  command \"${TZ_BIN_DIR}/tz-bin\" \"\$@\"
  local _r=\$?
  if [[ -f \"${TZ_CONFIG_DIR}/env.sh\" ]]; then
    # shellcheck source=/dev/null
    source \"${TZ_CONFIG_DIR}/env.sh\"
  fi
  return \$_r
}
${TZ_MARKER_END}"

  local path_snippet="${TZ_PATH_MARKER}
export PATH=\"${TZ_BIN_DIR}:\${PATH}\"
${TZ_PATH_MARKER_END}"

  _tz_append_snippet() {
    local rc="$1"
    local content="$2"
    [[ -f "${rc}" ]] || return 0
    if grep -q "${TZ_MARKER}" "${rc}" 2>/dev/null && grep -q "${content%%$'\n'*}" "${rc}" 2>/dev/null; then
      echo ">> ${rc} 已有 tz 配置，跳过"
      return 0
    fi
    # 移除旧块后重写
    if grep -q "${TZ_MARKER}" "${rc}" 2>/dev/null; then
      sed -i "/${TZ_MARKER}/,/${TZ_MARKER_END}/d" "${rc}"
    fi
    printf '\n%s\n' "${content}" >> "${rc}"
    echo ">> 已配置 ${rc}"
  }

  _tz_append_snippet "${HOME}/.bashrc" "${snippet}"
  _tz_append_snippet "${HOME}/.zshrc" "${snippet}"

  _tz_append_path() {
    local rc="$1"
    [[ -f "${rc}" ]] || return 0
    if grep -q "${TZ_PATH_MARKER}" "${rc}" 2>/dev/null; then
      return 0
    fi
    if grep -q '${HOME}/.local/bin' "${rc}" 2>/dev/null || grep -q '.local/bin' "${rc}" 2>/dev/null; then
      return 0
    fi
    printf '\n%s\n' "${path_snippet}" >> "${rc}"
    echo ">> 已添加 PATH 到 ${rc}"
  }

  _tz_append_path "${HOME}/.bashrc"
  _tz_append_path "${HOME}/.zshrc"

  mkdir -p "${TZ_CONFIG_DIR}"
}

tz_install_binary() {
  local src="$1"
  mkdir -p "${TZ_BIN_DIR}"
  install -m 755 "${src}" "${TZ_BIN_DIR}/tz-bin"
  echo ">> tz 已安装到 ${TZ_BIN_DIR}/tz-bin"
}

tz_detect_arch() {
  local arch
  arch="$(uname -m)"
  case "${arch}" in
    x86_64|amd64) echo "x86_64-unknown-linux-gnu" ;;
    aarch64|arm64) echo "aarch64-unknown-linux-gnu" ;;
    *) echo "unsupported:${arch}" >&2; return 1 ;;
  esac
}

tz_print_done() {
  cat <<EOF

✓ tz 安装完成

  命令: tz
  配置: ${TZ_CONFIG_DIR}/env.sh

请执行一次:
  source ~/.bashrc    # 或 source ~/.zshrc

功能:
  · 系统代理 (浏览器) + 终端 http_proxy/all_proxy
  · 退出 tz 后当前 shell 自动生效

代理服务 (Mihomo) 需单独部署:
  git clone https://github.com/soul667/proxy_server.git
  cd proxy_server   # 创建 proxy.yaml 填入节点
  docker compose up -d

EOF
}
