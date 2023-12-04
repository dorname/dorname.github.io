---
title: 走读Writing an OS in Rust实验（二）
date: 2023-12-02
extra:
    image: ../rust.jpg
taxonomies:
  tags:
    - Rust
  authors:
    - liguangqiao  
---

# 独立式可执行程序

（[参考自作者phil的官方博客](https://os.phil-opp.com/zh-CN/freestanding-rust-binary/#jin-yong-zhan-zhan-kai)）

为了用 Rust 编写一个操作系统内核，我们需要创建一个独立于操作系统的可执行程序。这样的可执行程序常被称作**独立式可执行程序**（freestanding executable）或**裸机程序**(bare-metal executable)。

构建裸机程序主要需要五步：

1. 禁用标准库
2. 重新实现`panic`处理函数
3. 禁用栈展开（事实上重写程序入口）
4. 重写程序入口
5. 编译成裸机目标

## 禁用标准库

目标：断开与标准库的链接，使用核心库脱离操作系统绑定

`#![no_std]`添加到程序可以断开与标准库的链接

```rust
#![no_std]

fn main() {
    println!("Hello, world!");
}
```

`cargo build`验证

1. 发现`println!`宏已经找不到了。

```
error: cannot find macro `println` in this scope
 --> src/main.rs:4:5
  |
4 |     println!("Hello, world!");
  |     ^^^^^^^

error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`
```

2. 去掉`println!("Hello, world!");`后重新build

```
error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`
```

## 实现panic处理函数

目标：解决恐慌处理器函数缺失错误

```rust
use core::panic::PanicInfo;

/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

`cargo build`验证

```
error: language item required, but not found: `eh_personality`
```

**注意： eh_personality 语言项**

语言项是一些编译器需求的特殊函数或类型。举例来说，Rust 的 [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html) trait 是一个这样的语言项，告诉编译器哪些类型需要遵循**复制语义**（[copy semantics](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html)）——当我们查找 `Copy` trait 的[实现](https://github.com/rust-lang/rust/blob/485397e49a02a3b7ff77c17e4a3f16c653925cb3/src/libcore/marker.rs#L296-L299)时，我们会发现，一个特殊的 `#[lang = "copy"]` 属性将它定义为了一个语言项，达到与编译器联系的目的。

我们可以自己实现语言项，但这是下下策：目前来看，语言项是高度不稳定的语言细节实现，它们不会经过编译期类型检查（所以编译器甚至不确保它们的参数类型是否正确）。幸运的是，我们有更稳定的方式，来修复上面的语言项错误。

`eh_personality` 语言项标记的函数，将被用于实现**栈展开**（[stack unwinding](https://www.bogotobogo.com/cplusplus/stackunwinding.php)）。在使用标准库的情况下，当 panic 发生时，Rust 将使用栈展开，来运行在栈上所有活跃的变量的**析构函数**（destructor）——这确保了所有使用的内存都被释放，允许调用程序的**父进程**（parent thread）捕获 panic，处理并继续运行。但是，栈展开是一个复杂的过程，如 Linux 的 [libunwind](https://www.nongnu.org/libunwind/) 或 Windows 的**结构化异常处理**（[structured exception handling, SEH](https://docs.microsoft.com/en-us/windows/win32/debug/structured-exception-handling)），通常需要依赖于操作系统的库；所以我们不在自己编写的操作系统中使用它。

## 禁用栈展开

目标：解决语言项找不到的问题

`Cagro.toml`文件添加配置项禁用panic的栈展开（包含开发环境和发布版本）

```
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

`cargo build`验证，已没有语言项的提示

```
error: requires `start` lang_item
```

显示缺少start语言项

## 重写程序入口

目标：解决start语言项缺少问题

处理方案：不使用预定义的入口（`main`），重新编写一个函数作为操作系统入口

`#![no_main]`属性可以禁用预定入口，此时main函数已经可以安全删除

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

重新编写一个函数`_start`

**注意：函数名为 `_start` ，是因为大多数系统默认使用这个名字作为入口点名称**

```rust
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
```

`_start`作为操作系统入口需要注意3点：

1. 确保Rust编译器正确输出一个名字为_start的函数，`#[no_mangle]`标记的作用是**禁用名称重整**——这确保Rust编译器输出一个名为`_start`的函数；否则，编译器可能最终生成名为`_ZN3blog_os4_start7hb173fedf945531caE`的函数，无法让链接器正确识别。
2. 函数标记为`extern "C"`，告诉编译器这个函数应当使用C语言的调用约定。而不是Rust语言的调用约定。
3. 设置返回值类型为永不返回类型。在 Rust 中，`!` 表示一个特殊的类型，称为 "never" 类型。这个类型表示一个永远不会返回的表达式或函数。通常，`!` 类型用于表示 panic 或者无限循环等永远不会正常结束的操作。 这是必需的，因为入口点不由任何函数调用，而是由操作系统或引导加载程序直接调用。 因此，入口点不应返回，而应调用一些特殊函数，例如操作系统的退出系统函数。 在我们的例子中，这函数中实现关闭机器可能是一个合理的操作，因为如果独立的二进制文件返回，则没有什么可做的。 现在，我们通过无限循环来满足要求。

`cargo build`验证

```
error: linking with `cc` failed: exit status: 1
```

## 编译成裸机目标

目标：编译成裸机可执行程序

编译前首先需要待解决链接器错误。链接器是将生成的代码组合成可执行文件的程序。 由于 Linux、Windows 和 macOS 之间的可执行格式不同，因此每个系统都有自己的链接器，会引发不同的错误。 错误的根本原因是相同的：链接器的默认配置假设我们的程序依赖于 C 运行时，但事实并非如此。 为了解决这些错误，我们需要告诉链接器它不应该包含 C 运行时。 我们可以通过将一组特定的参数传递给链接器或构建裸机目标来做到这一点。

我是基于linux的，故仅仅介绍linux的编译参数

```
cargo rustc -- -C link-arg=-nostartfiles
```

​	这条命令使用了 `cargo` 命令来调用 `rustc` 编译器，并传递了一些额外的参数给编译器。让我们逐步解释这个命令：

1. **`cargo rustc`：** 这部分使用 `cargo` 工具来调用 Rust 编译器 `rustc`。`cargo rustc` 命令允许你向底层的 Rust 编译器传递额外的参数。
2. **`--`：** 这是一个分隔符，表示 `cargo` 命令的选项结束，后面的内容应该传递给底层的编译器。在这种情况下，`--` 之后的内容将被传递给 `rustc`。
3. **`-C link-arg=-nostartfiles`：** 这是传递给 `rustc` 的具体参数。这个参数告诉编译器在链接阶段使用 `-nostartfiles` 选项。`-nostartfiles` 是告诉链接器不使用标准的启动文件（start files）的选项。启动文件通常包含了程序启动前的初始化代码，现在要禁用这些初始化。
