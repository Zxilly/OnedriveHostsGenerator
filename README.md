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
