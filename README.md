# Proxy Server

基于 Mihomo (Clash Meta) 的多端口代理服务，每个端口固定出口地区。

## 端口分配

| 端口 | 地区 | 协议 |
|------|------|------|
| 1081 | 香港 | HTTP/SOCKS5 |
| 1082 | 日本(东京) | HTTP/SOCKS5 |
| 1083 | 美国(硅谷) | HTTP/SOCKS5 |
| 1084 | 新加坡 | HTTP/SOCKS5 |
| 1085 | 美国(弗吉尼亚) | HTTP/SOCKS5 |
| 1086 | 家宽 | HTTP/SOCKS5（经香港连家宽） |
| 9090 | 管理面板 | HTTP |

## 部署

```bash
# 1. 准备节点文件
cp proxy.yaml.example proxy.yaml
# 编辑 proxy.yaml 填入你的节点信息

# 2. 启动
docker compose up -d

# 3. 管理面板
# 浏览器打开 http://<服务器IP>:9090/ui/
# 密码: changeme
```

## 客户端配置 (bashrc)

在 `~/.bashrc` 或 `~/.zshrc` 中添加：

```bash
# 代理地址 (改成你的服务器 IP)
PROXY_HOST="127.0.0.1"

# 按需选择地区
alias proxy-hk="export https_proxy=http://${PROXY_HOST}:1081 http_proxy=http://${PROXY_HOST}:1081 all_proxy=socks5://${PROXY_HOST}:1081"
alias proxy-jp="export https_proxy=http://${PROXY_HOST}:1082 http_proxy=http://${PROXY_HOST}:1082 all_proxy=socks5://${PROXY_HOST}:1082"
alias proxy-us="export https_proxy=http://${PROXY_HOST}:1083 http_proxy=http://${PROXY_HOST}:1083 all_proxy=socks5://${PROXY_HOST}:1083"
alias proxy-sg="export https_proxy=http://${PROXY_HOST}:1084 http_proxy=http://${PROXY_HOST}:1084 all_proxy=socks5://${PROXY_HOST}:1084"
alias proxy-va="export https_proxy=http://${PROXY_HOST}:1085 http_proxy=http://${PROXY_HOST}:1085 all_proxy=socks5://${PROXY_HOST}:1085"
alias proxy-jk="export https_proxy=http://${PROXY_HOST}:1086 http_proxy=http://${PROXY_HOST}:1086 all_proxy=socks5://${PROXY_HOST}:1086"
alias proxy-off="unset https_proxy http_proxy all_proxy"
```

使用：

```bash
source ~/.bashrc

proxy-hk        # 切换到香港
curl ip.sb      # 验证出口 IP

proxy-us        # 切换到美国
proxy-off       # 关闭代理
```

## 节点文件格式

`proxy.yaml` 只需包含 `proxies:` 列表：

```yaml
proxies:
  - name: "香港-节点名"
    type: vless
    server: x.x.x.x
    port: 443
    uuid: xxx
    # ...其他参数

  - name: "东京-节点名"
    type: vless
    server: x.x.x.x
    port: 443
    uuid: xxx
    # ...
```

节点名中需包含对应地区关键词（香港/东京/硅谷/新加坡/弗吉尼亚），config 通过 filter 自动匹配。

## tz 交互式代理切换 (TUI)

Rust 编写的终端界面，一键同时设置 **系统代理 + 终端代理**。

### 一键安装（推荐）

```bash
curl -fsSL https://raw.githubusercontent.com/soul667/proxy_server/main/scripts/install.sh | bash
source ~/.bashrc   # 或 source ~/.zshrc
tz
```

从 GitHub Release 下载预编译包（`tz-latest`），自动配置 `~/.bashrc` / `~/.zshrc`。

### 本地开发安装

```bash
git clone https://github.com/soul667/proxy_server.git
cd proxy_server && ./scripts/install-tz.sh
source ~/.bashrc
```

切换后会同时生效：
- **系统代理**（gsettings）→ 浏览器等桌面应用
- **终端代理**（`http_proxy` / `all_proxy`）→ curl、git 等

终端代理写入 `~/.config/tz/env.sh`，**退出 tz 后当前 shell 自动 source**。

操作说明：
- `↑/↓` 或 `j/k` 选择
- `Enter` 确认（设置代理 / 查看当前 / 测速 / 关闭）
- `q` 退出

功能：
- 同时设置系统代理 + 终端环境变量
- 设置香港/日本/美国/新加坡/弗吉尼亚/家宽
- 查看当前代理状态
- 六个端口测速对比（出口 IP + 延迟）
- 关闭系统代理

## 文件说明

```
├── docker-compose.yml   # 容器编排
├── config.yaml          # Mihomo 配置（listener + proxy-group）
├── proxy.yaml           # 节点信息（不入库，.gitignore）
├── Dockerfile           # 构建镜像
├── scripts/             # install.sh, install-tz.sh, proxy-switch.sh
├── scripts/lib/         # tz 安装公共库
├── tz/                  # Rust TUI 源码
└── .github/workflows/   # CI：Docker 镜像 + tz 预编译包
```
