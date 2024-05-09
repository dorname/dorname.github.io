---
title: Fisco Bcos链
date: 2024-03-08
extra:
    image: ../blockchain_one.jpg
taxonomies:
  tags:
    - blockchain
    - bcos
  authors:
    - liguangqiao
---

# Fisco Bcos

## 国密链部署

### 新机构接入已有链

新节点接入所需要的文件：

- 证书机构颁发的国密证书（需要从外部获取）

- 证书机构生成的链证书（需要从外部获取）

- 群组创世文件（需要从已有的区块链网路节点中取）`group.1.genesis`

- 群组1节点地址`{ip:port}`

- `generator`二进制文件

- `fisco-bcos`2.8.0版本二进制文件

- `conf`配置目录

- 模板目录`tpl`（用完可以删掉）

  ```sh
  .
  ├── applicationContext.xml
  ├── config.ini
  ├── config.ini.gm
  ├── download_console.sh
  ├── group.i.genesis
  ├── group.i.ini
  ├── load_new_groups.sh
  ├── reload_whitelist.sh
  ├── start_all.sh
  ├── start.sh
  ├── stop_all.sh
  └── stop.sh
  ```

- `pys` 脚本目录

- `log`日志目录

脚本执行步骤：

1. 将已有链的证书拷贝到新机构的目录下，这里分别将非国密证书和国密证书文件夹命名为`chain_ca` 、`chain_ca_gm`。

2. 利用链证书生成新机构的国密证书和私钥以及链证书。

   ```sh
   ./generator --generate_agency_certificate ./agency_ca ./chain_ca kingdom
   ./generator --generate_agency_certificate ./agency_ca_gm ./chain_ca_gm kingdom -g
   ```

   ```sh
   agency_ca/
   └── kingdom
       ├── agency.crt 机构非国密证书
       ├── agency.key 机构私钥
       └── ca.crt 链证书
   1 directory, 3 files
   
   agency_ca_gm/
   └── kingdom
       ├── gmagency.crt 机构非国密证书
       ├── gmagency.key 机构国密私钥
       └── gmca.crt 国密链证书
   1 directory, 3 files
   ```

3. 拷贝到`meta`目录下：

   ```sh
   cp  ./agency_ca/kingdom/* ./meta
   cp  ./agency_ca_gm/kingdom/* ./meta
   ```

4. 修改节点信息配置文件`node_deployment.ini`。

   主要包含所在群组、机构的节点以及每个节点的p2p地址、jrpc地址和channel地址。

5. 生成节点信息

   ```sh
    ./generator --generate_all_certificates ./node_info -g
   ```

   ```sh
   node_info/
   ├── gmcert_127.0.0.1_30300.crt 节点证书
   ├── gmcert_127.0.0.1_30301.crt 节点证书
   └── peers.txt 机构所有节点地址
   ```

6. 拷贝群组创世文件到`meta`目录下

7. 生成机构所属节点

   ```sh
   ./generator --build_install_package ./node_info/peers.txt ./nodes -g
   ```

8. 修改节点配置文件config.ini中bcos的版本为2.8.0

   ```sh
   sed -i "s/2.9.0/2.8.0/g" nodes/node_127.
   0.0.1_3030*/config.ini
   ```

9. 启动节点

   ```sh
    bash nodes/start_all.sh
   ```

**注意：**

- 新节点加入群组：

  1）需要拷贝文件群组创世文件。

  2）通过控制台或者webase将节点接入共识节点/游离节点。

## 异地组网实验

### 公网实验环境准备

1. 两台异网设备（这里是为了真实模拟外网组网环境，实际上本机上把连接的节点地址都换成对应的本机公网地址也可以）。

2. `fiscobcos`版本为`2.8.0`。本人本地链已经搭建完毕，本地链在设备1

3. 从本地链的目录中找到`cert`、`gmcert`、二进制文件`fisco-bcos`、`start.sh`、`stop.sh`以及任意node目录下的`config.ini`以及`node/conf`下的`group.1.genesis`和`group.1.ini`，拷贝备用，我这里拷贝到了设备2的`backup`目录·。

4. 下载节点生成脚本`gen_node_cert.sh`参考[官方文档](https://fisco-bcos-documentation.readthedocs.io/zh-cn/latest/docs/tutorial/add_new_node.html?highlight=新增节点)，构建项目结构

   ```
   .
   ├── cert
   ├── fisco-bcos
   ├── gen_node_cert.sh
   ├── gmcert
   ```

   生成节点`bash gen_node_cert.sh -c ./cert/agencyA -o node -g ../gmcert/agencyA-gm/`

   ```
   .
   ├── cert
   ├── fisco-bcos
   ├── gen_node_cert.sh
   ├── gmcert
   └── node
   ```

5. 拷贝文件到对应的目录

   ```sh
   cp ../backup/config.ini ./node
   cp ../backup/group.1.* ./node/conf/
   cp ../backup/*.sh ./node/
   ```

6. 设备1的本地链选取至少一个节点服务基于`cpolar`做的内网穿透，获取穿透后的公网ip，得到的公网ip格式为`{协议}://{ip}:{端口号}`如`tcp://x.SSS.SSSSS.SS:xxxxx`，ip是做过字符转换处理的，可以通过`telnet {ip}{port}`知道真实的ip：

   ```bash
   $ telnet x.SSS.SSSSS.SS xxxxx
   Trying xxx.xx.xx.xxx...
   Connected to x.SSS.SSSSS.SS.
   Escape character is '^]'.
   Connection closed by foreign host.
   ```

   **说明：**

   1. `x`代表数值
   2. `S`代表字符

   **注意：**` xxx.xx.xx.xxx`为真实ip，解释一下为什么需要获取真实ip：因为我的另一台实验设备运行`fisco-bcos`文件时，对于`2.tcp.cpolar.top`域名类型处理总是报错，而另一台设备则没有问题，初步判断是Ubuntu 22..04 和20版本在某些细节处理不一样，而`bcos`恰恰用了。需要真实错误原因，需要进行调试方可知晓。

7. 配置config.ini

   ```
   [p2p]
       listen_ip=0.0.0.0
       listen_port=30306
       node.1=xxx.xx.xx.xxx:xxxxx
       node.4=127.0.0.1:30306
   ```

### 实验过程

1. 启动设备1的本地链

2. 启动设备2的节点`bash start.sh`

3. 检查设备2的日志

   ```
   [P2P][Service] heartBeat,connected count=1
   ```

   说明节点已经与设备1本地链的节点建立了连接。

### 实验结果

只要公网通的情况下，组网肯定没问题。

### 其他问题

现在需要理解清楚handshakeClient和handshakeServer的意思，对于节点存在何种影响。

目前经常出现的错误就是handshakeClient失败

## 参考文档

1. [扩容一个新节点 — FISCO BCOS 2.0 v2.9.0 文档 (fisco-bcos-documentation.readthedocs.io)](https://fisco-bcos-documentation.readthedocs.io/zh-cn/latest/docs/tutorial/add_new_node.html?highlight=新增节点)