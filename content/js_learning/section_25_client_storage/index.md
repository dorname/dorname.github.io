---
title: js学习笔记(25)
date: 2023-05-23
extra:
    image: ../js_logo.png
taxonomies:
  tags:
    - js
  authors:
    - liguangqiao  
---
## Section 25 客户端存储

js为什么会涉及到客户端存储呢？

**目标**：将不常更新、且相对公用的数据存储在客户端中，减少与服务端的交互，提高整体的性能。

## cookie

原始目标：
要求HTTP头部包含会话信息。

### cookie具有局限性

**浏览器级别的局限性：**

1. 不超过300个cookie
2. 每个cookie不超过4096字节
3. 每个域不超过20个cookie
4. 每个域不超过81920字节。

**域级别的局限性：**

| 浏览器         | 每个域的cookie限制 |
| -------------- | ------------------ |
| Edge           | 不超过50           |
| Firefox        | 不超过150个        |
| Opera          | 不超过180个        |
| Safari和Chrome | 没有硬性限制       |

**浏览器对于cookie的过限处理：**

如果cookie总数超过了单域上限，浏览器会删除之前设置的cookie。

- Opera的处理策略是最少使用删除原则
- Firefox是随机删除原则

### cookie构成

![cookie_arc](cookie_arc.png)

- 名称：cookie的唯一可标识，且不区分大小写，cookie名称必须经过URL编码。

- 值：存储在cookie里的字符串值，值也必须经过URL编码

- 域：cookie的有效域。发送到这个域的所有请求都会包含对应的cookie

- 路径：请求URL中包含这个路径才会把cookie发送到服务器。

- 过期时间：表示何时删除cookie的时间戳。

  ​	删除策略：

  - 默认会话结束后删除cookie
  - 定时删除cookie
  
- 安全标记：设置之后，只在使用SSL安全连接的情况下才会把cookie发送到服务器。例如:*https://www.test.com*会发送cookie，而请求*http://www.wrox.com*则不会。

### js的cookie

js中的cookie接口只有*BOM(Brower Object Model)*的document.cookie

取值过程会获取页面中所有的有效cookie字符串（根据域、路径、过期时间和安全设置），以分号分隔。

### 子cookie

子cookie的机制实际上是利用cookie的机制，将一连串的cookie信息编码成字符串，给cookie的其中一个键上。

### 使用cookie的注意事项

叫做**HTTP-only**的cookie。**Http-only**可以在浏览器设置，也可以在服务器设置。但只能在服务器上读取。这种cookie值js无法读取。

**注意**：因为所有cookie都会作为请求头部由浏览器发送给服务器，所以在cookie种保存大量信息可能影响特定域浏览器请求的性能。保存的cookie越大，请求完成的时间就越长。即使浏览器对cookie大小有限制，最好还是尽可能只能保存必要信息，避免性能问题。

对cookie的限制及其特性决定了cookie并不是存储大量数据的理想方式。
