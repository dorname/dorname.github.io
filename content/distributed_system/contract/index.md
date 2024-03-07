---
title: 智能合约
date: 2024-03-07
extra:
    image: ../logo.png
taxonomies:
  tags:
    - blockchain
  authors:
    - liguangqiao
---

# 智能合约

主要以Fisco Bcos 和Hyperleger Fabric为主进行合约的学习和研究。

## Fisco Bcos合约

### Solidity合约

#### revert错误处理

`revert`指令的作用是回滚合约。

合约的交易回执中，若状态码为`0x16`,状态信息为`RevertInstruction`，表示执行了回滚指令revert。

#### 地址传递

##### 状态变量修改函数的地址传递

例如：

现有合约TestAdd.sol、Test.sol，Test 导入了TestAdd并调用了它的test方法。

已知部署Test 的用户地址为0xe2d3a5f33454452081eb2e4ba0f99f97e40fc840。

合约地址为0x7d26aec47df6c3702c91898b05fec3a2efd0fd2f。

用户调用合约的parent方法，当前合约的`msg.sender`地址是用户地址0xe2d3a5f33454452081eb2e4ba0f99f97e40fc840。

那么调用test的`msg.sender`应该是合约地址还是发起交易用户地址？

其中用到了两个solidity的官方api，包含`msg.sender`当前消息发送者、tx.origin交易原始调用者。

使用错误处理回滚api`revert`打印结果显示为合约地址:0x7d26aec47df6c3702c91898b05fec3a2efd0fd2f。

`msg.sender`的地址具有传递性，并非调用链上和原始调用者保持一致，而是遵循调用关系，比如说，用户调用合约A方法则合约A中的`msg.sender`为用户公钥地址，合约A方法调用了合约B方法，则合约B中的`msg.sender`为合约A的地址，依此类推。

`tx.origin`在整个调用链上始终保持一致，表示为最终发起交易的用户地址。

**Test**

```solidity
//Test.sol
pragma solidity ^0.4.25;
import "./TestUtil.sol";
//字符串存储数据结构
contract Test {
    TestUtil t = new TestUtil();
    event print(address,address);
    function parent() public{
        emit print(tx.origin,msg.sender);
        t.test();
    }
}
```

**TestUtil**

```solidity
//TestUtil.sol
pragma solidity ^0.4.25;
//字符串存储数据结构
contract TestUtil {
    function test() public returns(bool re){
        
        address a = 0xe2d3a5f33454452081eb2e4ba0f99f97e40fc840;
        if(msg.sender!=a){
             revert(addressToString(msg.sender));
        }
        return true;
    }
    function addressToString(address _addr) public pure returns(string memory) {
        bytes32 value = bytes32(uint256(uint160(_addr)));
        bytes memory alphabet = "0123456789abcdef";
        bytes memory str = new bytes(42); // 因为以太坊地址长度为20字节，字符串表示为40个字符加上"0x"前缀
        str[0] = '0';
        str[1] = 'x';
        for (uint256 i = 0; i < 20; i++) {
            str[2+i*2] = alphabet[uint8(value[i + 12] >> 4)];
            str[3+i*2] = alphabet[uint8(value[i + 12] & 0x0f)];
        }
        return string(str);
    }
    constructor() public{
        
    }
}
```

##### view修饰的查询函数地址传递

`msg.sender`只需要考虑一种特殊情况的随机性，就是部署当前合约之后，通过一个view修饰的方法返回`msg.sender`时，每次交易的结果返回的`msg.sender`值是不一样的。去调view的修饰，则与当前发起交易的用户地址保持一致。

验证环境：

ide:webase v1.5.4 

bcos:v2.8.0

合约内容如下：

```solidity
pragma solidity ^0.4.25;
contract TestSubC {
    function getWithView() public view returns(address){
        return msg.sender;
    }
    function getWithoutView() public returns(address){
        return msg.sender;
    }
}
```

#### 自定义修饰器（modifier）

函数处理前执行自定义逻辑

```solidity
modifer check() {
	//自定义逻辑
	_;
}
```

函数处理后执行自定义逻辑

```solidity
modifer check() {
	_;
	//自定义逻辑
}
```

#### 修改器

- `pure`修饰函数时表示：不允许修改或访问状态。

  适用于业务处理和计算。

- `view`修饰函数时：不允许修改状态。

  适用于查询

##### 钩子

初步测试solidity支持钩子，可以通过预留合约钩子来方便后续合约代码的拓展。

子合约继承后，可以通过复写父合约的的钩子方法做额外的处理。

```solidity
function preTest() internal {}
function afterTest() internal {}
function test() public {
	 preTest();
	//实际测试逻辑
	afterTest();
}
```

##### 版本限制

**v0.4.25版本**

- 目前重写函数时没有约束，父合约不需要`virtual`,子合约不需要`override`修饰。

##### 知识点

- address(this) 获取当前合约地址
- 前置后置等公共逻辑可以抽象成modifer

##### 思考

- 合约调用层级的权限控制是否只能通过modifier去做？暂时没有其他方式，始终都是需要做权限验证的。

- 需要如何避免重复部署导致数据层，数据丢失的问题。

  写一个table，存放数据层合约地址（前提是数据不直接存储在table中）

- 重写方法，可见性修饰得保持一致。

## Hyperleger Fabric

