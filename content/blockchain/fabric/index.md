---
title: Hyperledger Fabric
date: 2024-05-11
extra:
    image: ../blockchain_one.jpg
taxonomies:
  tags:
    - blockchain
    - fabric
  authors:
    - liguangqiao
---

# 超级账本

## 专业术语和概念

- [Assets](https://hyperledger-fabric.readthedocs.io/en/latest/fabric_model.html#assets)— Asset definitions enable the exchange of almost anything with monetary value over the network, from whole foods to antique cars to currency futures.

  ​	灵活的资产定义，从食物、汽车甚至未来货币，这些定义的资产可以在网络中流通。

  **详细描述：**资产可以是实体的（如房地产和硬件）也可以是无形的（如合同和知识产权）。Hyperledger Fabric 提供了使用链码（chaincode）交易修改资产的能力。

  ​	在 Hyperledger Fabric 中，资产被表示为一组键值对的集合，状态变化被记录在通道账本（Channel ledger）上的交易中。资产可以以二进制和/或 JSON 形式表示。

- [Chaincode](https://hyperledger-fabric.readthedocs.io/en/latest/fabric_model.html#chaincode) — Chaincode execution is partitioned from transaction ordering, limiting the required levels of trust and verification across node types, and optimizing network scalability and performance.

  链码的执行和交易排序是解耦的，这限制了不同节点类型之间所需的信任和验证级别，并优化了网络的可扩展性和性能。

  **详细描述：**链码是定义一个或多个资产以及修改这些资产的交易指令的软件；换句话说，它就是业务逻辑。链码强制执行读取或更改键值对或其他状态数据库信息的规则。链码函数针对账本的当前状态数据库执行，并通过交易提案来启动。链码执行的结果生成一组键值写入（写集），这些写集可以提交到网络，并在所有对等节点上应用到账本上。

- [Ledger Features](https://hyperledger-fabric.readthedocs.io/en/latest/fabric_model.html#ledger-features) — The immutable, shared ledger encodes the entire transaction history for each channel, and includes SQL-like query capability for efficient auditing and dispute resolution.

  账本具有不可变性、公开透明性，账本是对每个通道交易历史编码的结果，其中包括了类似SQL的查询能力，以实现高效的审计和争议解决。

  **详细描述：**账本是 Hyperledger Fabric 中记录所有状态转换的有序、防篡改的记录。状态转换是由参与方提交的链码调用（即“交易”）的结果。每一笔交易都会生成一组资产键值对，这些键值对作为创建、更新或删除操作被提交到账本中。

  账本由以下两部分组成：

  1. **区块链（chain）**：用于以区块的形式存储不可变的有序记录。
  2. **状态数据库**：用于维护当前的 Fabric 状态。
     - 每个通道（channel）都有一个账本。
     - 每个对等节点（peer）都维护着其所属通道的账本副本。

  以下是 Hyperledger Fabric 账本的一些特点：

  - **基于键的查询和更新**：可以使用基于键的查找、范围查询和复合键查询来查询和更新账本。
  - **使用丰富的查询语言进行只读查询**：如果使用 CouchDB 作为状态数据库，可以使用丰富的查询语言进行只读查询。
  - **只读历史查询**：可以查询键的账本历史记录，支持数据来源场景。
  - **交易组成**：交易包含链码读取的键/值版本（读取集 read set）和链码写入的键/值（写入集 write set）。
  - **交易签名和排序服务**：交易包含每个背书对等节点的签名，并提交给排序服务。
  - **区块排序和分发**：交易被排序进区块，并由排序服务“分发”到通道上的对等节点。
  - **交易验证和策略执行**：对等节点根据背书策略验证交易并执行这些策略。
  - **版本检查**：在追加区块之前，执行版本检查以确保自链码执行以来，被读取的资产的状态没有变化。
  - **交易的不可变性**：一旦交易被验证并提交，就具有不可变性。
  - **配置区块**：通道的账本包含一个定义策略、访问控制列表和其他相关信息的配置区块。
  - **成员服务提供者实例**：通道包含成员服务提供者（Membership Service Provider, MSP）实例，允许从不同的证书颁发机构派生加密材料。

- [Privacy](https://hyperledger-fabric.readthedocs.io/en/latest/fabric_model.html#privacy) — Channels and private data collections enable private and confidential multi-lateral transactions that are usually required by competing businesses and regulated industries that exchange assets on a common network.

  通道和私有数据集合使得在共同网络上交换资产的竞争性企业和受监管行业能够进行私密且保密的多边交易。这些特性通常是由这些行业所需的。

  **详细描述：**

  ​	Hyperledger Fabric 通过每个通道上的不可变账本以及可以操作和修改资产当前状态（即更新键值对）的链码来实现其功能。账本存在于通道的范围内 —— 它可以被整个网络共享（假设每个参与者都在一个共同的通道上操作），或者它可以被私有化，只包括一组特定的参与者。

  在后一种情况下，这些参与者将创建一个单独的通道，从而隔离/分隔他们的交易和账本。为了解决希望在完全透明和隐私之间架起桥梁的场景，链码只能安装在需要访问资产状态以执行读取和写入的对等节点上（换句话说，如果链码没有安装在对等节点上，它将无法正确地与账本接口）。

  ​	当通道上的一组组织需要保持他们的交易数据机密时，可以使用私有数据集合（集合）将这些数据隔离在私有数据库中，逻辑上与通道账本分开，只能由授权的组织子集访问。

  ​	因此，通道使交易对更广泛的网络保持私密，而集合则在通道上的组织子集之间保持数据私密。

  ​	为了进一步混淆数据，链码中的值可以在发送交易到排序服务并将区块附加到账本之前，使用常见的加密算法（如AES）部分或全部加密。一旦加密数据被写入账本，只有拥有用于生成密文的相应密钥的用户才能解密。

  ​	以下是 Hyperledger Fabric 中关于账本、通道和私有数据集合的一些关键点：

  1. **通道（Channel）**：是 Fabric 网络中的一个独立通信通道，允许组织之间进行私密通信和交易。

  2. **账本（Ledger）**：每个通道都有一个账本，记录了该通道上所有交易的不可变序列。

  3. **链码（Chaincode）**：是定义资产和交易规则的智能合约，可以安装在需要与账本交互的对等节点上。

  4. **私有数据集合（Private Data Collection）**：允许组织在通道内创建一个私有的数据空间，只有被授权的组织才能访问。

  5. **数据加密**：在交易提交到账本之前，可以使用加密算法对数据进行加密，以增强数据的安全性。

  6. **数据解密**：只有拥有正确密钥的用户才能解密加密的数据。

     通过这些机制，Hyperledger Fabric 提供了灵活的方式来处理网络中的隐私和保密问题，同时保持了区块链技术的核心优势，如数据不可篡改性和透明性。

- [Security & Membership Services](https://hyperledger-fabric.readthedocs.io/en/latest/fabric_model.html#security-membership-services) — Permissioned membership provides a trusted blockchain network, where participants know that all transactions can be detected and traced by authorized regulators and auditors.

  许可制成员资格提供了一个受信任的区块链网络，在该网络中，参与者知道所有交易都可以被授权的监管机构和审计员检测和追踪。

  **详细描述：**

  Hyperledger Fabric 支持一个交易网络，其中所有参与者都具有已知的身份。使用公钥基础设施（Public Key Infrastructure, PKI）生成与组织、网络组件以及最终用户或客户端应用程序相关的加密证书。因此，可以在更广泛的网络层面和通道层面上操作和控制数据访问控制。Hyperledger Fabric 的这种“许可制”概念，结合通道的存在和能力，有助于解决隐私和保密性至关重要的场景。

  以下是 Hyperledger Fabric 中关于身份验证、权限控制和通道的关键点：

  1. **已知身份**：网络中的每个参与者都有一个已知的身份，这有助于建立信任和责任。

  2. **公钥基础设施 (PKI)**：用于创建加密证书，这些证书是数字身份的形式，与参与者的身份紧密相关。

  3. **数据访问控制**：通过 PKI 生成的证书，可以对谁可以访问网络中的哪些数据进行细粒度控制。

  4. **网络层面的控制**：可以在整个网络范围内设置访问控制，以管理不同组织和用户对网络资源的访问。

  5. **通道层面的控制**：通道允许在特定组织之间创建私有通信路径，通道上的访问控制可以独立于网络的其他部分进行设置。

  6. **许可制区块链**：与开放给任何人参与的“非许可制”区块链不同，Hyperledger Fabric 是一个许可制区块链，意味着所有参与者都必须经过身份验证和授权。

  7. **隐私和保密性**：通道和私有数据集合提供了机制，以确保只有授权的参与者才能访问特定的交易数据和资产状态。

  8. **智能合约（链码）**：链码可以包含权限检查，确保只有具有适当权限的参与者可以执行特定的交易。

  9. **成员服务提供者 (MSP)**：是 Hyperledger Fabric 中用于身份验证和授权的组件，每个组织都有自己的 MSP。

  通过这些特性，Hyperledger Fabric 能够为需要高度隐私和保密性的企业级应用提供强大的区块链解决方案。它允许在保持数据安全和完整性的同时，灵活地处理不同参与者之间的交易和数据共享。

- [Consensus](https://hyperledger-fabric.readthedocs.io/en/latest/fabric_model.html#consensus) — A unique approach to consensus enables the flexibility and scalability needed for the enterprise.

  一种独特的共识方法为企业提供了所需的灵活性和可扩展性。

  **详细描述：**

  在分布式账本技术中，共识（consensus）一词已经不仅仅与特定算法或单一功能同义，它的含义更为广泛，涵盖了确认交易顺序之外的多个方面。Hyperledger Fabric 通过其在整个交易流程中的基本作用来强调这一区别，包括从提案和背书到排序、验证和提交的各个阶段。简而言之，共识被定义为对组成区块的一组交易的正确性进行全方位验证的过程。

  共识的达成最终是在区块中交易的顺序和结果满足了明确的策略标准检查时实现的。这些检查和平衡在交易的生命周期中进行，包括使用背书策略来指定必须背书特定类别交易的特定成员，以及系统链码（system chaincodes）来确保这些策略得到执行和维护。在提交之前，对等节点将使用这些系统链码来确保有足够的背书存在，并且它们来自适当的实体。此外，还将进行版本检查，即在将包含交易的区块附加到账本之前，对账本的当前状态进行确认或同意。这最后的检查提供了对双重支付操作和其他可能破坏数据完整性的威胁的保护，并允许对非静态变量执行函数。

  除了进行多种背书、有效性和版本检查外，交易流程的所有方向上还在持续进行身份验证。在网络的层次结构上（从排序服务到通道）实施了访问控制列表，并且随着交易提案通过不同的架构组件，有效载荷被反复地签名、验证和认证。总之，共识不仅仅局限于对一批交易顺序的同意；它是一个全面的特征，是在交易从提案到提交的旅程中不断进行的验证的副产品。

  以下是 Hyperledger Fabric 中共识过程的一些关键点：

  1. **交易提案**：交易由网络中的参与者提出，并作为交易流程的起点。

  2. **背书（Endorsement）**：根据背书策略，特定的对等节点对交易提案进行背书。

  3. **排序（Ordering）**：排序服务将背书的交易排序并打包成区块。

  4. **验证（Validation）**：对等节点验证交易的有效性，包括背书的充分性和策略的遵守。

  5. **提交（Commitment）**：经过验证的交易被提交到账本，区块被添加到区块链中。

  6. **版本检查**：在提交前确认账本的当前状态，以防止双重支付和其他威胁。

  7. **身份验证**：在整个交易流程中，持续进行身份验证以确保交易的安全性。

  8. **访问控制列表（ACL）**：在网络的不同层次上实施，以控制对资源的访问。

  9. **签名和验证**：交易提案在通过不同架构组件时，其有效载荷被签名和验证。

  共识过程是确保 Hyperledger Fabric 网络中交易正确性和数据完整性的关键机制，它通过一系列复杂的检查和验证来实现。

