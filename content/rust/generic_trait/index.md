---
title: rust学习(三)泛型和特征对象
date: 2023-07-15
extra:
    image: ../rust.jpg
taxonomies:
  tags:
    - Rust
  authors:
    - liguangqiao  
---

# **泛型和特征对象**

***泛型解决了多种数据类型可共用一个方法的问题，而特征对象（trait）则提供一种定义和限制泛型行为的方法，特征对象(trait)中封装各类型共享的方法***

泛型例子：

```rust
use std::ops::Add;

fn add_i8(a:i8,b:i8) -> i8 {
    a+b
}
fn add_i16(a:i16,b:i16) -> i16 {
    a+b
}
fn add_i32(a:i32,b:i32) -> i32 {
    a+b
}
fn add<T:std::ops::Add + Add<Output = T>>(a:T,b:T) -> T {
    a+b
}
fn add_test(){
    println!("{}",add_i8(1,2));
    println!("{}",add_i16(1,2));
    println!("{}",add_i32(1,2));
    println!("{}",add(1,2));
}
#[test]
fn test() {
    add_test();

}
```



**注意：** 一个*trait*只能由方法、类型、常量三部分组成，它描述了一种抽象接口。接口的使用方式，实现*trait*，默认为类型实现公共方法。

例如：

```rust
use std::fmt;
//定义一个工人结构体
struct worker{
    name:String,
    workerId:i32,
    
}
//实现调试打印特征
impl fmt::Debug for worker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Worker")
            .field("name", &self.name)
            .field("workerId", &self.workerId)
            .finish()
    }
}
//实现worker自我介绍方法
impl  worker {
    fn self_produce(&self){
        println!("{}",self.name);
    }
    
}
//定义行为特征
trait behavior{
    //新类型
    type Tool;
    //常量
    const YEAR:i32 = 2023;
    fn start(&self) {
        println!("start working!!!");
    }
    fn work(&self){
        println!("coding");
    }
    fn end(&self){
        println!("end working!!!");
    }
    //泛型例子，仅实现了fmt::Debug特征的类型可使用此函数
    fn talking_shit<T:fmt::Debug>(&self,name: &T,msg: &T){
        println!("{:?}is ^*** {:?}",name,msg);
    }
    fn init_tool(&self,x: Self::Tool);
}

impl behavior for worker{
    // 确认新类型
    type Tool = String;
  	//重载方法
    fn init_tool(&self,x: Self::Tool){
        println!("{:?},{:?}",Self::YEAR,x);
    }
}
#[test]
fn test(){
    let w = worker{
        name:String::from("mengzhongshadouyou"),
        workerId:123
    };
    w.init_tool(String::from("getting keybroad"));
    w.start();
    w.work();
    w.end();
    w.self_produce();
    w.talking_shit(&String::from("boss"), &String::from("idiot"));
    println!("{:?}",w);
}
```







