---
title: rust学习(二)所有权和生命周期
date: 2023-07-15
extra:
    image: ../rust.jpg
taxonomies:
  tags:
    - Rust
  authors:
    - liguangqiao  
---
# Rust所有权机制

## Rust语言引入所有权机制的目标

主要为解决复杂系统中的资源管理和悬吊引用问题。

**所有权系统**是***Rust***中用于管理内存的手段。

所有权规则总结归纳：

- 每个值（变量）都有一个所有者
- 值在任意时刻都只有一个所有者
- 当所有离开作用域时，其值将被丢弃(相当于执行垃圾回收)

结论：***Rust***的值被唯一的对象管理着，一旦不使用，内存就会立刻释放。

所有权简单实验

1. 借用

   ```rust
   let x = String::from("hello");
   let y = &x;
   ```

2. 克隆

   ```rust
   let x = 10;
   let y = x;
   println!("测试通过{x},{y}");
   //测试通过10，10
   let a = "hello";
   let b = a;
   println!("测试通过{x},{y}");
   let c = (1,2 as f64,"hello");
   let d = c;
   println!("测试:{:?},{:?}",c,d);
   let g = d.clone();
   ```

生命周期

静态生命周期***'static***和动态生命周期***'a***

静态生命周期在整个程序中期间存活

动态生命周期，相关参数的生命周期以相对长生命周期的引用为准

```rust
fn lifetime_test_one() -> &String{
        let x= "hello".to_string();
        print!("{x}");
        &x
}
//error missing liftime...
//改成使用一个生命周期都声明为'a的参数y来承接x的值
fn lifetime_test_one<'a>(y:&'a mut String) -> &String{
        let x:String = String::from("hello");
        print!("{x}");
        *y = x;
        y
}
```

