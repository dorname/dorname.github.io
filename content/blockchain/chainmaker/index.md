---
title: 初识长安链
date: 2024-03-07
extra:
    image: ../blockchain_one.jpg
taxonomies:
  tags:
    - blockchain
  authors:
    - liguangqiao
---

# 长安链（chainmaker）

## 四节点docker离线部署

### 环境准备

我的实验环境是麒麟服务器外加一台可联网的机器（没有arm架构机器可以考虑使用`qemu-static-user`，安装之后允许本地跑`arm`架构的镜像，虽然会报警告，但没有影响运行）

要求联网可以通过ssh远程访问服务器。

**目标是在服务器上部署四节点的CERT证书模式的长安链。**

#### 1 服务器docker 和docker-compose 离线安装

参考[麒麟系统（arm64/aarch64）离线安装docker和docker-compose_docker-compose-linux-aarch64-CSDN博客](https://blog.csdn.net/qq_23845083/article/details/130768859)

#### 2 准备好长安链的arm64镜像。

联网环境：

`docker pull chainmakerofficial/chainmaker:v2.3.2_arm`

`docker save -o chainmaker.tar chainmakerofficial/chainmaker:v2.3.2_arm`

可以得到一个`chainmaker.tar`本地镜像包，并拷贝到服务器中（这里我是使用`scp`命令）

服务器：

`docker load -i chainmaker.tar`

#### 3 准备长安链的配置文件

详细参考官方文档，只需要准备到**2.1.2.4. 账户生成**

**注意：国密版本需要修改配置模板参考官方文档**

[2. 通过命令行体验链 — chainmaker-docs v2.3.2 documentation](https://docs.chainmaker.org.cn/v2.3.2/html/quickstart/通过命令行体验链.html)

```
# 查看生成好的节点密钥和配置
$ tree -L 3 ../build/
../build/
├── config
│   ├── node1
│   │   ├── admin
│   │   ├── chainconfig
│   │   ├── chainmaker.yml
│   │   ├── log.yml
│   │   ├── node1.key
│   │   ├── node1.nodeid
│   │   ├── node1.pem
│   │   └── user
│   ├── node2
│   │   ├── admin
│   │   ├── chainconfig
│   │   ├── chainmaker.yml
│   │   ├── log.yml
│   │   ├── node2.key
│   │   ├── node2.nodeid
│   │   ├── node2.pem
│   │   └── user
│   ├── node3
│   │   ├── admin
│   │   ├── chainconfig
│   │   ├── chainmaker.yml
│   │   ├── log.yml
│   │   ├── node3.key
│   │   ├── node3.nodeid
│   │   ├── node3.pem
│   │   └── user
│   └── node4
│       ├── admin
│       ├── chainconfig
│       ├── chainmaker.yml
│       ├── log.yml
│       ├── node4.key
│       ├── node4.nodeid
│       ├── node4.pem
│       └── user
├── crypto-config
│   ├── node1
│   │   ├── admin
│   │   ├── node1.key
│   │   ├── node1.nodeid
│   │   ├── node1.pem
│   │   └── user
│   ├── node2
│   │   ├── admin
│   │   ├── node2.key
│   │   ├── node2.nodeid
│   │   ├── node2.pem
│   │   └── user
│   ├── node3
│   │   ├── admin
│   │   ├── node3.key
│   │   ├── node3.nodeid
│   │   ├── node3.pem
│   │   └── user
│   └── node4
│       ├── admin
│       ├── node4.key
│       ├── node4.nodeid
│       ├── node4.pem
│       └── user
└── crypto_config.yml  
```

其中config目录用于节点组网

crypto_config目录用于后续sdk接入

#### 4 编写docker-compose.yml文件和启动脚本

可以参考`chainmaker-go`中的`scripts/docker/v2.3.0/four-nodes.docker-compose.yml`去写

```yaml
version: '3'
services:
  node1:
    container_name: cm-node1
    hostname: cm-node1
    image: "chainmakerofficial/chainmaker:v2.3.2_arm"
    restart: always
    working_dir: "/chainmaker-go/bin"
    command: "./chainmaker start -c ../config/wx-org1.chainmaker.org/chainmaker.yml > panic.log"
    ports:
      - "11301:11301"
      - "12301:12301"
    volumes:
      - ../config/four-nodes/wx-org1.chainmaker.org:/chainmaker-go/config/wx-org1.chainmaker.org
      - ./tmp/data1:/chainmaker-go/data
      - ./tmp/log1:/chainmaker-go/log
    networks:
      chainmakerofficial-localnet:
        aliases:      
          - cm-node1
        ipv4_address: 172.49.0.11
  node2:
    container_name: cm-node2
    hostname: cm-node2
    image: "chainmakerofficial/chainmaker:v2.3.2_arm"
    restart: always
    working_dir: "/chainmaker-go/bin"
    command: "./chainmaker start -c ../config/wx-org2.chainmaker.org/chainmaker.yml > panic.log"
    ports:
      - "11302:11302"
      - "12302:12302"
    volumes:
      - ../config/four-nodes/wx-org2.chainmaker.org:/chainmaker-go/config/wx-org2.chainmaker.org
      - ./tmp/data2:/chainmaker-go/data
      - ./tmp/log2:/chainmaker-go/log
    networks:
      chainmakerofficial-localnet:
        aliases:      
          - cm-node2
        ipv4_address: 172.49.0.12
  node3:
    container_name: cm-node3
    hostname: cm-node3
    image: "chainmakerofficial/chainmaker:v2.3.2_arm"
    restart: always
    working_dir: "/chainmaker-go/bin"
    command: "./chainmaker start -c ../config/wx-org3.chainmaker.org/chainmaker.yml > panic.log"
    ports:
      - "11303:11303"
      - "12303:12303"
    volumes:
      - ../config/four-nodes/wx-org3.chainmaker.org:/chainmaker-go/config/wx-org3.chainmaker.org
      - ./tmp/data3:/chainmaker-go/data
      - ./tmp/log3:/chainmaker-go/log
    networks:
      chainmakerofficial-localnet:
        aliases:      
          - cm-node3
        ipv4_address: 172.49.0.13
  node4:
    container_name: cm-node4
    hostname: cm-node4
    image: "chainmakerofficial/chainmaker:v2.3.2_arm"
    restart: always
    working_dir: "/chainmaker-go/bin"
    command: "./chainmaker start -c ../config/wx-org4.chainmaker.org/chainmaker.yml > panic.log"
    ports:
      - "11304:11304"
      - "12304:12304"
    volumes:
      - ../config/four-nodes/wx-org4.chainmaker.org:/chainmaker-go/config/wx-org4.chainmaker.org
      - ./tmp/data4:/chainmaker-go/data
      - ./tmp/log4:/chainmaker-go/log
    networks:
      chainmakerofficial-localnet:
        aliases:      
          - cm-node4
        ipv4_address: 172.49.0.14

networks:
  chainmakerofficial-localnet:
    driver: bridge
    ipam:
      config:
        - subnet: 172.49.0.0/16
```

#### 5  创建服务器项目目录和修改文件

- 创建chainmaker `mkdir chainmaker`

- 把准备好的文件材料放在`chainmaker`目录下包含`docker-compose.yml`文件、`config`目录、`crypto-config`目录
- 修改docker-compose.yml文件

​	主要修改`      - ../config/four-nodes/wx-org1.chainmaker.org:/chainmaker-go/config/wx-org1.chainmaker.org`成当前config文件目录`      - ./config/node1:/chainmaker-go/config/wx-org1.chainmaker.org`。节点2、3、4一样。

- 修改config目录下每一个node的chainmaker.yml文件

  修改`seeds`因为我们是四个容器所以这部分修改成动态的配置如下：

      - "/dns/cm-node1/tcp/11301/p2p/QmfMMsPqkiQ53CWENvCMqo4cLv8mJXzsbJ3t7tnpp81ito"
      - "/dns/cm-node2/tcp/11302/p2p/Qmb22Taf6EJJVuwVu3pRXHTCx3g8JPRFEY8FC1hYxLUSCm"
      - "/dns/cm-node3/tcp/11303/p2p/QmRgmmXzGUjauHwD3xCDeApebBY7s2tedezutvzYJ6SAH6"
      - "/dns/cm-node4/tcp/11304/p2p/QmZK1RbHchgN5G61mAqw184AHqfUvtEXjEMNyH2uB3XyDs"

#### 6 执行脚本构建镜像，并运行容器

构建并启动

`docker-compose -f docker-compose.yml up -d`

关闭容器

`docker-compose -f docker-compose.yml down --volumes`

查看容器运行 `docker ps`

观察内部日志`docker logs {容器id}`

## solidity合约部署

### 合约编译

需要借助solcjs插件进行本地编译

## CMC工具合约部署

进入chainmaker-go/tools/cmc编译（注意需要走交叉编译，因为涉及到在`x86架构`编译`arm架构`的可执行文件）

本人是直接跑了一个镜像容器，通过文件夹映射的方式，把镜像中现成的cmc工具拿了出来，放到了chainmaker目录下。

1. chainmaker目录下创建目录`mkdir testdata`

2. 把x需要的测试合约拷贝到`testdata`下,在源码项目`chainmaker-go/tools/cmc/testdata`

3. 拷贝`crypto-config`到`testdata`目录下

4. 创建`sdk_config.yml`

   ```yaml
   chain_client:
     # 链ID
     chain_id: "chain1"
     # 组织ID
     org_id: "wx-org1.chainmaker.org"
     # 客户端用户私钥路径
     user_key_file_path: "./testdata/crypto-config/wx-org1.chainmaker.org/user/client1/client1.tls.key"
     # 客户端用户私钥密码(无密码则不需要设置)
   #  user_key_pwd: "123"
     # 客户端用户证书路径
     user_crt_file_path: "./testdata/crypto-config/wx-org1.chainmaker.org/user/client1/client1.tls.crt"
     # 客户端用户加密私钥路径(tls加密证书对应私钥，应用于国密GMTLS双证书体系；若未设置仅使用单证书）
     user_enc_key_file_path: "./testdata/crypto-config/wx-org1.chainmaker.org/user/client1/client1.tls.enc.key"
     # 客户端用户加密私钥密码(无密码则不需要设置)
   #  user_enc_key_pwd: "123"
     # 客户端用户加密证书路径(tls加密证书，应用于国密GMTLS双证书体系；若未设置仅使用单证书）
     user_enc_crt_file_path: "./testdata/crypto-config/wx-org1.chainmaker.org/user/client1/client1.tls.enc.crt"
     # 客户端用户交易签名私钥路径(若未设置，将使用user_key_file_path)
     user_sign_key_file_path: "./testdata/crypto-config/wx-org1.chainmaker.org/user/client1/client1.sign.key"
     # 客户端用户交易签名私钥密码(无密码则不需要设置)
   #  user_sign_key_pwd: "123"
     # 客户端用户交易签名证书路径(若未设置，将使用user_crt_file_path)
     user_sign_crt_file_path: "./testdata/crypto-config/wx-org1.chainmaker.org/user/client1/client1.sign.crt"
     # 同步交易结果模式下，轮询获取交易结果时的最大轮询次数，删除此项或设为<=0则使用默认值 10
     retry_limit: 20
     # 同步交易结果模式下，每次轮询交易结果时的等待时间，单位：ms 删除此项或设为<=0则使用默认值 500
     retry_interval: 500
     # 当前签名证书的别名。当设置此配置项时，chain client 对象将自动检查链上是否已添加此别名，如果没有则自动上链此证书别名，
     # 并且后续所有交易都会使用别名，别名可降低交易体大小。若为空则不启用。
   #  alias: my_cert_alias
     # txid配置项：默认支持TimestampKey，如果开启enableNormalKey则使用NormalKey
     enable_normal_key: false
   
     enable_tx_result_dispatcher: true
   
     nodes:
       - # 节点地址，格式为：IP:端口:连接数
         node_addr: "127.0.0.1:12301"
         # 节点连接数
         conn_cnt: 10
         # RPC连接是否启用双向TLS认证
         enable_tls: true
         # 信任证书池路径
         trust_root_paths:
           - "./testdata/crypto-config/wx-org1.chainmaker.org/ca"
         # TLS hostname
         tls_host_name: "chainmaker.org"
     archive:
       # 数据归档链外存储相关配置
       # 如果使用了新版本的归档中心,这个地方配置为archivecenter
       type: "mysql"  # archivecenter 归档中心, mysql mysql数据库    
       dest: "root:123456:localhost:3306"
       secret_key: xxx
     rpc_client:
       max_receive_message_size: 100 # grpc客户端接收消息时，允许单条message大小的最大值(MB)
       max_send_message_size: 100 # grpc客户端发送消息时，允许单条message大小的最大值(MB)
       send_tx_timeout: 60 # grpc 客户端发送交易超时时间
       get_tx_timeout: 60 # rpc 客户端查询交易超时时间
     pkcs11:
       enabled: false # pkcs11 is not used by default
       library: /usr/local/lib64/pkcs11/libupkcs11.so # path to the .so file of pkcs11 interface
       label: HSM # label for the slot to be used
       password: 11111111 # password to logon the HSM(Hardware security module)
       session_cache_size: 10 # size of HSM session cache, default to 10
       hash: "SHA256" # hash algorithm used to compute SKI  
     # # 如果启用了归档中心,可以打开下面的归档中心配置  
     archive_center_query_first: true # 如果为true且归档中心配置打开,那么查询数据优先从归档中心查询 
     # archive_center_config:
     #   chain_genesis_hash: c670b598127f5795767d1acbae435e714797596f7e0a55dd05205da948de1a0a
     #   archive_center_http_url: http://127.0.0.1:13119
     #   request_second_limit: 10
     #   rpc_address: 127.0.0.1:13120
     #   tls_enable: false
     #   tls:
     #     server_name: archiveserver1.tls.wx-org.chainmaker.org
     #     priv_key_file: ./testdata/archivecenter/archiveclient1.tls.key
     #     cert_file: ./testdata/archivecenter/archiveclient1.tls.crt
     #     trust_ca_list:
     #       - ./testdata/archivecenter/ca.crt    
     #   max_send_msg_size: 200
     #   max_recv_msg_size: 200 
   ```

5. 创建合约

   ```bash
   ./cmc client contract user create \
   --contract-name=balance001 \
   --runtime-type=EVM \
   --byte-code-path=./testdata/balance-evm-demo/ledger_balance.bin \
   --abi-file-path=./testdata/balance-evm-demo/ledger_balance.abi \
   --version=1.0 \
   --sdk-conf-path=./testdata/sdk_config.yml \
   --admin-key-file-paths=./testdata/crypto-config/wx-org1.chainmaker.org/user/admin1/admin1.sign.key,./testdata/crypto-config/wx-org2.chainmaker.org/user/admin1/admin1.sign.key,./testdata/crypto-config/wx-org3.chainmaker.org/user/admin1/admin1.sign.key \
   --admin-crt-file-paths=./testdata/crypto-config/wx-org1.chainmaker.org/user/admin1/admin1.sign.crt,./testdata/crypto-config/wx-org2.chainmaker.org/user/admin1/admin1.sign.crt,./testdata/crypto-config/wx-org3.chainmaker.org/user/admin1/admin1.sign.crt \
   --sync-result=true
   ```

6. 调用合约

   ```bash
   ./cmc client contract user invoke \
   --contract-name=balance001 \
   --method=updateBalance \
   --sdk-conf-path=./testdata/sdk_config.yml \
   --params="[{\"uint256\": \"10000\"},{\"address\": \"0xa166c92f4c8118905ad984919dc683a7bdb295c1\"}]" \
   --sync-result=true \
   --abi-file-path=./testdata/balance-evm-demo/ledger_balance.abi
   ```

7. 查询合约

   ```bash
   ./cmc client contract user get \
   --contract-name=fact \
   --method=find_by_file_hash \
   --sdk-conf-path=./testdata/sdk_config.yml \
   --params="{\"file_hash\":\"ab3456df5799b87c77e7f88\"}"
   ```

更多的操作参考官方文档 [3. 长安链CMC工具（Cert） — chainmaker-docs v2.3.2 documentation](https://docs.chainmaker.org.cn/v2.3.2/html/dev/命令行工具.html)

## 常用配置文件路径

以节点1为例

节点区块链配置文件：`config/node1/chainconfig/bc1.yml`

节点证书配置文件：`config/node1/chainmaker.yml`

节点管理员证书文件目录:`config/node1/admin`

节点客户端证书文件目录`config/node1/user`

