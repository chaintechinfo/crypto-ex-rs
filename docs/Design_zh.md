# 设计文档

一个使用 Rust 开发的加密货币交易所，计划从交易所最核心的撮合引擎开始，一点点完善功能，最终可能不会把交易所涉及到的各种功能都实现出来，但会把最核心的几个链路实现出来，并进行优化和架构演进。

## 迭代

1. 撮合引擎
2. 下单 + 账户
3. 清算全回路
4. 市场行情与主动通知
5. 加密货币充值和提现
6. OTC
7. 拓展
  1. 杠杆
  2. 期货合约（永续和交割）
8. 金融业务
  1. DeFi
  2. 理财
  3. 支付
  4. 质押借币
9. NFT
10. ...

### Matching Engine

- V1 先实现 OrderBook 的基本功能，支持限价单
  - V1.1 增加对市价单的支持
  - V1.2 增加对 Stop Limit Order 的支持
  - V1.3 增加 IOC / FOK / Post-only 的支持
- V2 封装 OrderBook 对外提供 API 能力
  - V2.1 引入多币对，实现撮合的线程模型
  - V2.2 提供内部可调用的 API
  - V2.3 支持 MQ
- V3 支持基本的快照功能
- V4 提高可用性，每个币对撮合的竞争选主
  - V4.1 增加 zk 或者 etcd
  - V4.2 支持故障恢复，重放快照和订单序列
- V5 解决性能问题
  - V5.1 性能调优
  - V5.2 JVM 参数优化
- V6 升级架构，持续优化
  - 复制状态机方案