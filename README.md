# OnedriveHostsGenerator
A Page include Onenote for Windows 10's hosts

介绍: [一个自动维护的 Onedrive（UWP 版） hosts 列表](https://learningman.top/archives/245)

## 地址

`https://onedrive-hosts.learningman.top`

缓存每 6000 秒更新一次。

你可以附加 `ipv4` 或者 `ipv6` 作为查询参数来获取不同的结果。默认为全部输出。
附加 `single` 作为查询参数以使得每个域名只包含一个解析的 IP 地址。

## 开发

你可以在 [Office 365 端点](https://docs.microsoft.com/zh-cn/office365/enterprise/office-365-endpoints) 找到所有 `Microsoft 365 应用` 所使用的域名，但因为该文档中含有大量通配符域名，实践中需要配合本地客户端抓包

## Sponsor

CDN acceleration and security protection for this project are sponsored by Tencent EdgeOne.

<img src="https://github.com/user-attachments/assets/603e5752-74d5-43fc-84a9-1aa6a0ce0367" alt="描述" width="300"/>

## 验证依赖升级

使用当前稳定 Rust（Hickory 0.26 本身要求 Rust 1.88；当前锁文件中的依赖要求至少 1.89，验证使用当前稳定版）及提交的 Cargo.lock：

```sh
cargo fmt -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo check --locked
cargo test --locked --all
cargo build --locked --release
cargo tree --locked -i rustls-webpki
```

默认测试离线验证输出格式、单/双栈选择、未解析域名及加密上游配置。
允许访问公共 DNS 的开发环境中可另行运行网络测试：

```sh
cargo test --locked encrypted_upstreams_resolve_both_families -- --ignored --nocapture
```

该测试分别检查 Cloudflare、Quad9、Google 的 DoH（443）和 DoT（853），需要网络允许这两类连接；失败不能仅凭 CI 的出口限制认定是代码回归。
不会回退到系统 DNS 或明文 UDP/TCP。保留双栈解析、6 个并发请求、5 秒超时及 3 次尝试。
