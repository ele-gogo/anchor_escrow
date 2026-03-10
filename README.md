# Anchor Escrow

一个基于 Solana 区块链的去中心化托管（Escrow）协议，使用 Anchor 框架开发。

## 📖 项目简介

Blueshift Anchor Escrow 是一个智能合约协议，旨在通过区块链实现安全的点对点资产交换。该协议解决了链上交易中的信任问题，确保交易双方在满足条件后才完成代币转移。

### 核心功能

- **去中心化托管**：无需第三方中介的资产托管服务
- **条件支付**：只有在满足预定条件时才会释放资金
- **退款机制**：支持卖方在未成交时取回托管资产
- **安全可靠**：使用 PDA（程序派生地址）保证账户安全

## 🎯 使用场景

- 点对点代币交换
- 去中心化交易（DEX）中的订单匹配
- 需要第三方托管的交易场景
- 学习 Anchor 框架和 Solana 程序开发的示例项目

## 🏗️ 系统架构

### 主要指令（Instructions）

本合约包含三个核心指令：

1. **Make** - 创建托管订单
   - 卖方将资产锁定到托管账户
   - 设置接收条件和金额
   - 生成唯一的托管账户

2. **Take** - 接受订单
   - 买方支付约定金额的代币
   - 提取被托管的资产
   - 完成交易并关闭托管账户

3. **Refund** - 退款
   - 卖方在订单未被领取时取回资产
   - 关闭托管账户并返还代币

### 技术架构

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   Client    │────▶│  Escrow      │────▶│   Solana    │
│  (TS/JS)    │     │  Program     │     │   Runtime   │
└─────────────┘     └──────────────┘     └─────────────┘
                           │
                           ▼
                    ┌──────────────┐
                    │   PDA State  │
                    │  (On-chain)  │
                    └──────────────┘
```

### 目录结构

```
blueshift_anchor_escrow/
├── programs/
│   └── blueshift_anchor_escrow/
│       ├── src/
│       │   ├── instructions/      # 指令实现
│       │   │   ├── make.rs        # Make 指令
│       │   │   ├── take.rs        # Take 指令
│       │   │   ├── refund.rs      # Refund 指令
│       │   │   └── mod.rs         # 指令模块导出
│       │   ├── state.rs           # 状态定义
│       │   ├── errors.rs          # 错误定义
│       │   └── lib.rs             # 程序入口
│       └── Cargo.toml
├── tests/                         # 测试文件
├── migrations/                    # 部署脚本
├── Anchor.toml                    # Anchor 配置
└── package.json                   # Node.js 依赖
```

## 🛠️ 技术栈

- **智能合约**: Rust + Anchor Framework v0.32.1
- **测试框架**: TypeScript + Mocha + Chai
- **区块链**: Solana
- **包管理**: Yarn / npm

## 📦 安装与配置

### 前置要求

在开始之前，请确保已安装以下工具：

- **Rust** (最新稳定版)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Solana CLI**
  ```bash
  sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
  ```

- **Anchor CLI**
  ```bash
  cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
  ```

- **Node.js** >= 16
  ```bash
  # 推荐使用 nvm
  nvm install 16
  nvm use 16
  ```

### 环境搭建

1. **克隆项目**
   ```bash
   cd blueshift_anchor_escrow
   ```

2. **安装依赖**
   ```bash
   yarn install
   # 或
   npm install
   ```

3. **配置 Solana**
   ```bash
   # 配置到本地集群
   solana config set --localhost
   
   # 生成钱包密钥对（如果没有）
   solana-keygen new -o ~/.config/solana/id.json
   ```

4. **验证配置**
   ```bash
   solana config get
   ```

## 🚀 快速开始

### 1. 启动本地验证节点

打开一个新终端窗口：

```bash
solana-test-validator
```

### 2. 构建项目

```bash
anchor build
```

构建产物位于 `target/deploy/blueshift_anchor_escrow.so`

### 3. 部署合约

```bash
anchor deploy
```

部署后会生成新的 Program ID，请更新 `Anchor.toml` 和 `lib.rs` 中的 ID。

### 4. 运行测试

```bash
anchor test
```

## 📝 使用说明

### Make - 创建托管订单

```typescript
const tx = await program.methods
  .make(
    seed,      // u64: 种子值
    receive,   // u64: 期望接收的代币数量
    amount     // u64: 托管的代币数量
  )
  .accounts({
    maker: makerPublicKey,
    // ... 其他必要账户
  })
  .rpc();
```

### Take - 接受订单

```typescript
const tx = await program.methods
  .take()
  .accounts({
    taker: takerPublicKey,
    // ... 其他必要账户
  })
  .rpc();
```

### Refund - 退款

```typescript
const tx = await program.methods
  .refund()
  .accounts({
    maker: makerPublicKey,
    // ... 其他必要账户
  })
  .rpc();
```

## 🧪 测试

项目包含完整的单元测试套件，位于 `tests/` 目录。

### 运行所有测试

```bash
yarn run ts-mocha -p ./tsconfig.json -t 1000000 "tests/**/*.ts"
```

### 测试覆盖范围

- ✅ Make 指令功能测试
- ✅ Take 指令功能测试
- ✅ Refund 指令功能测试
- ✅ 边界条件测试
- ✅ 错误处理测试

## 🔧 开发命令

### 构建

```bash
anchor build
```

### 部署到本地

```bash
anchor deploy --provider.cluster localnet
```

### 部署到 Devnet

```bash
anchor deploy --provider.cluster devnet
```

### 部署到 Mainnet

```bash
anchor deploy --provider.cluster mainnet
```

### 查看日志

```bash
anchor logs --provider.cluster <cluster> <program_id>
```

## 📚 API 参考

### 账户结构

#### Escrow 账户 (PDA)

存储托管订单的状态信息：

- `seed`: u64 - 唯一标识符
- `maker`: PublicKey - 卖方地址
- `receive`: u64 - 期望接收的代币数量
- `amount`: u64 - 托管的代币数量

### 错误码

自定义错误类型定义在 `errors.rs` 中，包括：

- 金额不匹配错误
- 账户权限错误
- 状态不一致错误

## 🔒 安全性

### 安全特性

- ✅ **PDA 所有权**: 使用程序派生地址确保账户安全
- ✅ **账户验证**: 所有账户访问都经过 Anchor 约束校验
- ✅ **溢出保护**: Rust 和 Anchor 提供默认的数值溢出检查
- ✅ **原子操作**: 所有交易都是原子的，要么全部成功，要么全部失败

### 注意事项

⚠️ **生产环境部署前请注意**:

1. 替换占位符 Program ID (`22222222222222222222222222222222222222222222`)
2. 进行全面的审计测试
3. 在主网部署前先在 Devnet 充分测试
4. 考虑添加额外的业务逻辑验证

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License

## 🔗 相关链接

- [Anchor 官方文档](https://www.anchor-lang.com/)
- [Solana 官方文档](https://docs.solana.com/)
- [Rust 编程语言](https://www.rust-lang.org/)

## 📞 联系方式

如有问题或建议，请提交 Issue 或通过以下方式联系：

- GitHub Issues
- Discord 社区

---

**免责声明**: 本项目仅供学习和参考使用。在生产环境中使用前，请务必进行全面的安全审计和测试。
