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

## 文件说明

```
├── docker-compose.yml   # 容器编排
├── config.yaml          # Mihomo 配置（listener + proxy-group）
├── proxy.yaml           # 节点信息（不入库，.gitignore）
├── Dockerfile           # 构建镜像
└── .github/workflows/   # CI 自动构建推送镜像
```
