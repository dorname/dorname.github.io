---
title: WebAssembly Rust and ExtJs 实验
date: 2023-05-02
extra:
  image: new.jpg
taxonomies:
  tags:
    - Webssembly
    - ExtJs
    - Rust
  authors:
    - liguangqiao  
---
# 突发奇想:利用WebAssembly、Rust去将前端重器框架ExtJs进行迁移重构
- 思路1：从根拔起，利用wasm实现rust和js之间通信直接重写一边Ext.Base、Ext.Event等基础设施
- 思路2：初步简单整合，外部引入ext-all-debug.js框架，利用wasm实现rust和js之间通信获取Ext命名空间，然后逐个复写Ext函数