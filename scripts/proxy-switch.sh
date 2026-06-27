#!/usr/bin/env bash
# 切换 GNOME 系统代理（浏览器、大部分桌面应用会跟随）
# 用法: ./scripts/proxy-switch.sh [hk|jp|us|sg|va|jk|off]

set -euo pipefail

HOST="${PROXY_HOST:-127.0.0.1}"

set_proxy() {
  local port="$1"
  gsettings set org.gnome.system.proxy mode 'manual'
  gsettings set org.gnome.system.proxy.http host "$HOST"
  gsettings set org.gnome.system.proxy.http port "$port"
  gsettings set org.gnome.system.proxy.https host "$HOST"
  gsettings set org.gnome.system.proxy.https port "$port"
  gsettings set org.gnome.system.proxy.socks host "$HOST"
  gsettings set org.gnome.system.proxy.socks port "$port"
  gsettings set org.gnome.system.proxy ignore-hosts "['localhost', '127.0.0.0/8', '::1']"
  echo "系统代理已开启: ${HOST}:${port}"
}

off_proxy() {
  gsettings set org.gnome.system.proxy mode 'none'
  echo "系统代理已关闭"
}

case "${1:-}" in
  hk) set_proxy 1081 ;;
  jp) set_proxy 1082 ;;
  us) set_proxy 1083 ;;
  sg) set_proxy 1084 ;;
  va) set_proxy 1085 ;;
  jk) set_proxy 1086 ;;
  off) off_proxy ;;
  *)
    echo "用法: $0 {hk|jp|us|sg|va|jk|off}"
    echo "  hk=香港  jp=日本  us=美国  sg=新加坡  va=弗吉尼亚  jk=家宽  off=关闭"
    exit 1
    ;;
esac
