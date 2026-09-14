# redis-rs-ohos

> 基于 [redis-rs](https://github.com/redis-rs/redis-rs) 的 **HarmonyOS / OpenHarmony Redis 客户端 SDK**，通过 N-API（ohrs）桥接，为 ArkTS 提供高性能、类型安全的 Redis 操作能力。

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![HarmonyOS](https://img.shields.io/badge/HarmonyOS-NEXT-007dff)](https://www.harmonyos.com/)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)
[![关联项目](https://img.shields.io/badge/关联-OhRedisTool-blue)](https://github.com/lll-gr/OhRedisTool)

---

## 这是什么？

`redis-rs-ohos` 是上游 [redis-rs](https://github.com/redis-rs/redis-rs) 的 HarmonyOS 适配分支，在保留 redis-rs 完整协议实现的基础上，新增了 `redis-ohos` crate，将 Rust 侧的 Redis 客户端能力通过 N-API 暴露给 ArkTS / TypeScript 层。

它是鸿蒙原生应用 **[OhRedisTool](https://github.com/lll-gr/OhRedisTool)**（Redis 可视化管理工具）的底层 Native 核心，同时也可以独立集成到任何 HarmonyOS 工程中。

## 特性

- **完整 Redis 命令支持**：字符串、哈希、列表、集合、有序集合、Stream 等常用数据类型全覆盖
- **同步 + 异步 API**：基于 tokio 的异步连接与连接管理器，支持自动重连
- **高级连接模式**：TLS（rustls）、Redis Cluster、Sentinel 哨兵
- **类型安全**：构建时自动生成 TypeScript / ArkTS 类型声明（`index.d.ts`）
- **HarmonyOS 原生日志**：集成 HiLog，Rust 侧日志自动输出到 hilog
- **高性能**：底层基于久经生产考验的 redis-rs，release 构建启用 LTO
- **零 WebView**：纯 Native 动态库（`.so`），不依赖任何 JS 运行时

## 关联项目

| 项目 | 说明 |
| --- | --- |
| [OhRedisTool](https://github.com/lll-gr/OhRedisTool) | 鸿蒙原生 Redis 可视化管理工具（本 SDK 的上层应用） |
| [redis-rs](https://github.com/redis-rs/redis-rs) | 上游 Rust Redis 客户端库（本项目 fork 自它） |
| [ohrs](https://github.com/ohos-rs/ohrs) | OpenHarmony / HarmonyOS N-API 绑定框架 |

## 项目结构

```
redis-rs-ohos/
├── redis/                  # 上游 redis-rs 核心库（fork，保持同步）
├── redis-ohos/             # ⭐ HarmonyOS N-API 桥接 crate（本项目核心）
│   ├── src/
│   │   ├── lib.rs          # N-API 模块入口
│   │   ├── client.rs       # RedisClient 封装
│   │   ├── connection.rs   # 同步连接
│   │   ├── json_connection.rs
│   │   ├── cluster_client.rs    # Redis Cluster 客户端
│   │   ├── sentinel_client.rs   # Sentinel 哨兵客户端
│   │   ├── tls_config.rs        # TLS 配置
│   │   ├── types.rs        # 类型转换（RedisValue ⇄ JsValue）
│   │   └── native_log.rs   # HiLog 桥接
│   ├── examples/           # ArkTS 使用示例（.ets）
│   ├── scripts/            # 构建脚本（交叉编译、打包）
│   ├── Makefile            # 构建入口（make build-release / make install）
│   ├── QUICKSTART.md       # 快速上手指南
│   ├── README_CN.md        # redis-ohos 详细中文文档
│   ├── ADVANCED_FEATURES.md
│   ├── COMMAND_COVERAGE.md # 命令覆盖清单
│   └── STREAM_COMMANDS.md  # Stream 命令文档
├── ohrs_example/           # ohrs 框架示例（参考用）
├── redis-test/             # 测试
├── Cargo.toml              # workspace 根配置
└── README.md               # 本文件
```

## 快速开始

### 1. 环境准备

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 ohrs（HarmonyOS N-API 构建工具）
cargo install ohrs

# 添加 HarmonyOS 交叉编译目标
rustup target add aarch64-unknown-linux-ohos
```

> 还需要 DevEco Studio 内置的 HarmonyOS NDK，并配置好对应的环境变量（`OHOS_NDK_HOME` 等）。

### 2. 构建 SDK

```bash
cd redis-ohos

# 检查环境
make check

# 构建 release 版本（启用 LTO）
make build-release
```

构建产物输出到 `redis-ohos/harmonyos-build/`，包含：
- `arm64-v8a/libredis_ohos.so`（真机）
- `x86_64/libredis_ohos.so`（模拟器）
- `index.d.ts`（ArkTS 类型声明）
- `oh-package.json5`

### 3. 安装到 HarmonyOS 工程

```bash
# 方式 A：Make 一键安装（推荐）
make install OHOS_PROJECT_PATH=/path/to/your/harmonyos/project

# 方式 B：手动复制
cp -r harmonyos-build/* /path/to/your/project/entry/libs/
```

在工程的 `entry/oh-package.json5` 中添加依赖：

```json5
{
  "dependencies": {
    "libredis_ohos.so": "file:./libs"
  }
}
```

### 4. ArkTS 中使用

```typescript
import { RedisClient, initLogging } from 'libredis_ohos.so';

// 初始化日志（可选）
initLogging(0xD001000, 'MyApp');

// 创建客户端
const client = new RedisClient('redis://127.0.0.1:6379');

// 获取连接并执行命令
const conn = client.getConnection();
console.log(conn.ping()); // "PONG"

conn.set('hello', 'world');
console.log(conn.get('hello')); // "world"

// 哈希操作
conn.hset('user:1', 'name', '张三');
console.log(conn.hget('user:1', 'name')); // "张三"
```

更多示例见 [`redis-ohos/examples/`](redis-ohos/examples/)，详细文档见 [`redis-ohos/README_CN.md`](redis-ohos/README_CN.md) 与 [`redis-ohos/QUICKSTART.md`](redis-ohos/QUICKSTART.md)。

## 与上游 redis-rs 的关系

本项目 fork 自 [redis-rs/redis-rs](https://github.com/redis-rs/redis-rs)，`redis/` 目录保持与上游同步，所有 HarmonyOS 相关的改动集中在新增的 `redis-ohos/` crate 中，不侵入上游核心代码。上游的 MIT 许可证继续适用。

## 开发

```bash
# 构建核心库
cargo build --locked -p redis

# 构建 redis-ohos
cd redis-ohos && make build-release

# 代码检查
cargo clippy --all-features --all --tests --examples
```

## 许可证

[MIT](LICENSE) © lllgr

上游 redis-rs 同样基于 MIT 许可证。
