# Rustengan: 分布式系统学习项目

`rustengan` 是一个基于 Rust 的分布式系统学习项目，灵感来源于 [jonhoo's rustengan 项目](https://github.com/jonhoo/rustengan)。它通过实现一系列分布式系统挑战，帮助开发者理解分布式系统的核心概念，如消息传递、广播和一致性。项目使用 [Maelstrom](https://github.com/jepsen-io/maelstrom) 框架进行测试，模拟多节点环境。

## 项目结构

- **`src/lib.rs`**: 核心库，定义了分布式系统的基本组件，包括：
  - `Message`: 消息结构，用于节点间通信。
  - `Node` trait: 定义节点行为。
  - `main_loop`: 主事件循环，处理输入输出。
- **`src/bin/`**: 包含多个二进制目标，每个实现一个分布式系统挑战：
  - `echo.rs`: 回显节点，接收消息并原样返回。
  - `unique.rs`: 唯一 ID 生成节点，为每个请求生成唯一标识符。
  - `broadcast.rs`: 广播节点，实现消息在网络中的传播。

## 功能

1. **Echo 节点**:
   - 接收 `echo` 消息，返回相同的消息作为 `echo_ok`。
   - 用于测试基本消息传递。

2. **Unique ID 节点**:
   - 接收 `generate` 请求，返回一个唯一 ID（格式为 `node-id`）。
   - 展示分布式系统中唯一性生成的基础。

3. **Broadcast 节点**:
   - 接收 `broadcast` 请求，将消息传播给所有节点。
   - 支持 `read` 请求，返回已知消息集合。
   - 通过 `topology` 消息配置网络结构。
   - 使用八卦协议（Gossip Protocol）实现高效消息同步。

## 安装

### 依赖
- **Rust**: 安装最新稳定版（推荐通过 [rustup](https://rustup.rs/)）。
- **Maelstrom**: 用于测试分布式系统。
  - 下载最新版本：[Maelstrom Releases](https://github.com/jepsen-io/maelstrom/releases)。
  - 解压到本地目录，例如 `/mnt/data/maelstrom`。
  - 确保 Java 已安装（`java -version`）。
- **Cargo 依赖**: 项目使用以下库（已在 `Cargo.toml` 中配置）：
  - `anyhow`: 错误处理。
  - `serde`: JSON 序列化/反序列化。
  - `rand`: 随机数生成（用于八卦协议）。

### 构建
```bash
cd /mnt/data/rust/learn_rustengan/rustengan
cargo build --release
```
- 可执行文件生成在 `target/release/` 下，例如 `target/release/echo`。

## 使用方法

### 手动运行
每个节点可以通过 `cargo run` 手动测试，需要提供 JSON 格式的输入。

#### Echo 节点
```bash
cargo run --bin echo
```
输入：
```
{"src": "c1", "dest": "n1", "body": {"type": "init", "node_id": "n1", "node_ids": ["n1"], "msg_id": 1}}
{"src": "c1", "dest": "n1", "body": {"type": "echo", "echo": "hello", "msg_id": 2}}
```

#### Unique ID 节点
```bash
cargo run --bin unique
```
输入：
```
{"src": "c1", "dest": "n1", "body": {"type": "init", "node_id": "n1", "node_ids": ["n1"], "msg_id": 1}}
{"src": "c1", "dest": "n1", "body": {"type": "generate", "msg_id": 2}}
```

#### Broadcast 节点
```bash
cargo run --bin broadcast
```
输入：
```
{"src": "c1", "dest": "n1", "body": {"type": "init", "node_id": "n1", "node_ids": ["n1", "n2"], "msg_id": 1}}
{"src": "c1", "dest": "n1", "body": {"type": "topology", "topology": {"n1": ["n2"], "n2": ["n1"]}, "msg_id": 2}}
{"src": "c1", "dest": "n1", "body": {"type": "broadcast", "message": 42, "msg_id": 3}}
{"src": "c1", "dest": "n1", "body": {"type": "read", "msg_id": 4}}
```

### Maelstrom 测试
使用 Maelstrom 运行完整测试，模拟多节点环境。

#### Echo
```bash
/mnt/data/maelstrom/maelstrom test -w echo --bin /mnt/data/rust/learn_rustengan/rustengan/target/release/echo --node-count 1 --time-limit 10
```

#### Unique IDs
```bash
/mnt/data/maelstrom/maelstrom test -w unique-ids --bin /mnt/data/rust/learn_rustengan/rustengan/target/release/unique --node-count 2 --time-limit 10
```

#### Broadcast
```bash
/mnt/data/maelstrom/maelstrom test -w broadcast --bin /mnt/data/rust/learn_rustengan/rustengan/target/release/broadcast --node-count 2 --time-limit 20 --rate 10
```

## 项目特点

- **模块化**: 核心逻辑在 `lib.rs` 中，二进制目标实现具体功能。
- **容错**: 使用 `anyhow` 处理错误，确保健壮性。
- **八卦协议**: `broadcast` 节点通过定时八卦实现高效消息传播。
- **可扩展**: 易于添加新节点实现其他挑战。

## 已知限制与改进方向

1. **Unique ID 的全局唯一性**:
   - 当前实现（`node-id`）在多节点下可能重复。
   - 改进建议：使用 UUID 或时间戳加节点 ID。

2. **Broadcast 的优化**:
   - 八卦协议未处理 EOF 信号，可能导致线程泄漏。
   - 可以添加停止机制或动态调整传播频率。

3. **测试覆盖**:
   - 当前依赖手动输入或 Maelstrom，未来可添加单元测试。

## 贡献

欢迎提交 Pull Request 或 Issue！如果你想实现新的挑战（比如 `g-counter` 或 `pn-counter`），可以：
1. 在 `src/bin/` 下添加新文件。
2. 更新 `Cargo.toml` 的 `[[bin]]` 部分。
3. 实现 `Node` trait。

## 许可证

本项目采用 MIT 许可证，详情见 [LICENSE](LICENSE) 文件（建议自行添加）。

---

### 使用说明
1. 将这个内容保存为 `README.md`，放在项目根目录（`/mnt/data/rust/learn_rustengan/rustengan/`）。
2. 如果你的二进制文件名不同（比如不是 `echo`、`unique`、`broadcast`），在 README 中调整对应的名称。
3. 如果需要更详细的安装步骤或代码示例，告诉我，我可以补充！

这个 README 应该能清楚地介绍你的项目给别人，同时也为你自己提供一个参考。有什么想调整的地方吗？