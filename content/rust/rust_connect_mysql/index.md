---
title: rust连接mysql建表测试
date: 2023-05-29
extra:
    image: ../rust.jpg
taxonomies:
  tags:
    - Rust
    - MySql
    - DataBase
  authors:
    - liguangqiao
---
# rust连接mysql建表测试

## 前期准备

- 创建一个新的***Rust***项目

  ```cargo
  cargo new web_server_demo
  ```

- 修改***Cargo.toml***

  1. 添加actix-web依赖，快速构建接口服务。
  2. 添加mysql依赖，连接/操作mysql数据库。
  3. 添加serde库依赖，用来序列化结构体实例。

  ```cargo
  [dependencies]
  actix-web = "4"
  mysql = "*"# 通配符*表示可以使用任何版本，通常会拉取最新版本
  serde = { version = "1.0", features = ["derive"] }
  ```

- 创建一个模块mysql_connect目录结构为

```rust
src
|--mysql_connect
	|--mod.rs
	|--database_test.rs
|--main.rs
```

mod.rs的内容

```rust
//导出模块database_test
pub mod database_test;
//导入模块database_test所有函数
pub use database_test::*;
```

- 接下是模块database_test.rs的内容

```rust
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mysql::{PooledConn, prelude::Queryable};
use serde::Deserialize;
const DATA_SOURCE_URL:&str = "mysql://root:lgq1995@localhost:3306/example";
// #[derive(Debug)]
#[derive(Deserialize,Debug)]
struct Info{
    name:String
}
//建表接口
#[get("/create_table")]
async fn create_table(info:web::Query<Info>) -> impl Responder{
    println!("name:{:?}",info.name);
    //Step 1 连接数据库
    let mut conn = connect();
    //Step 2 定义建表语句
    //TEMPORARY建一个永久表（非临时表）
    let create_table_sql= format!(r"CREATE TABLE {} (
             test_id int not null,
             username text,
             password text
        )",info.name);
    println!("{}",create_table_sql);
    //Step 3 执行建表方法
    let re  = conn.query_drop(create_table_sql);
    println!("{:?}",re.is_ok());
    //Step 4 根据执行结果，返回请求体
    if re.is_ok() {
        HttpResponse::Ok().body("建表成功")
    }else{
        HttpResponse::Ok().body("建表失败")
    }
}
fn connect()->PooledConn{
    use mysql::{Pool, PooledConn, prelude::*};
    let url = DATA_SOURCE_URL;
    // 连接数据库服务
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    // 返回数据库连接
    conn
}  
```

- 主模块中***main.rs***中的内容

```rust
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
//使用模块mysql_connect
mod mysql_connect;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
       .service(mysql_connect::database_test::create_table)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

```

访问8080端口，查看响应体中的建表信息

