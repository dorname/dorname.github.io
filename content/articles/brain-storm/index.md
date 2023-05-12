---
title: 小李很爱瞎想想
date: 2023-05-12
extra:
    image: brain.jpg
taxonomies:
  tags:
    - One Mind
  authors:
    - liguangqiao
---
#   做一套适用于所有view框架的数据缓存管理

灵感来源：我基于Ext的store做了一套缓存机制，实现了列表翻页缓存、为翻页新增提供了支撑。

想法：做一个纯js的前台数据管理机制，快速接入前端的各种view框架。

希望以黑盒的模式提供接口、达到可插拔的特性。
