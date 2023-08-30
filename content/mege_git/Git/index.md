---
title: git原理
date: 2023-08-30
extra:
    image: ../git.png
taxonomies:
  tags:
    - js
  authors:
    - liguangqiao  
---

# **Git原理学习**

## Git对象

***Git***是一个内容寻址文件系统。其核心思想就是键值对数据库（***key-value data store***），其核心行为包括：

1. 通过向 ***Git*** 仓库中插入任意类型的内容，它会返回一个唯一的键***(key)***。
2. 通过该键可以在任意时刻再次取回该内容。

### 实验出真知

可以通过底层命令 `git hash-object` 来演示上述效果——该命令可将任意数据保存于 .git/objects 目录（即 **对象数据库**），并返回指向该数据对象的唯一的键。

1. 初始化一个***Git***仓库

   ```
   $ git init git_lab
   ```

2. 检查一下***git***对象目录(`.git/objects`)生成了哪些文件夹，通过`find`命令查看所有文件夹（包含子文件夹）

   ```
   $ find .git/objects/
   .git/objects/
   .git/objects/info
   .git/objects/pack
   ```

3. 检查一下`.git/objects`目录下生成了哪些文件，同样通过`find`+文件路径+类型参数`-type f`查看所有文件

   ```
   $ find .git/objects/ -type f
   //无输出表示没有文件
   ```

4. 接下来要开始创建并保存一个文件了，这里简单创建一个文本文件

   ```
   echo "git testing" > git_test.txt
   ```

5. 保存到***git***对象目录（`.git/objects`）。这里使用了`git hash-object`命令，根据文件`git_test.txt`的内容生成一串`hash`值

   ```
   git hash-object -w git_test.txt
   34e75148c0aeb4efb212f817606d386b15190525
   ```

   **注意：hash值仅与文件内容有关，与名称无关。**

   ```
   $ echo "git testing" | git hash-object -w --stdin
   34e75148c0aeb4efb212f817606d386b15190525
   ```

6. 再次查看***git***对象目录（`.git/objects`）下的所有文件

   ```
   $ find .git/objects/ -type f
   .git/objects/34/e75148c0aeb4efb212f817606d386b15190525
   ```

   其中`hash`值长度40，前两个字符用于子目录命名，后38个字符用于文件命名。***hash算法参考SHA-1***

7. 文件`git_test.txt`内容已经存储到对象数据库中，如果需要从***git***中取回数据，则需要通过`cat-file`命令实现（个人理解`cat`实际就是`catch`的缩写），其中指定 `-p` 选项可指示该命令自动判断内容的类型，并为我们显示大致的内容

   ```
   $ git cat-file -p 34e75148c0aeb4efb212f817606d386b15190525
   git testing
   ```

8. 版本变更实验：更新文件`git_test.txt`的内容并重新保存

   ```
   $ echo "changing" > git_test.txt
   $ git hash-object -w git_test.txt
   372991728e3348b2c32b8dd5377885f2f77173ff
   ```

9. 查看对象数据库生成文件内容，它记录了该文件的两个不同版本。

   ```
   $ find .git/objects/ -type f
   .git/objects/34/e75148c0aeb4efb212f817606d386b15190525
   .git/objects/37/2991728e3348b2c32b8dd5377885f2f77173ff
   ```

10. 删除`git_test.txt`文件，然后尝试回归到第一个版本

    ```
    $ rm git_test.txt
    $ git cat-file -p 34e75148c0aeb4efb212f817606d386b15190525 > git_test.txt
    ```

**这里面其实有个问题：记住文件的每一个版本所对应的 SHA-1 值并不现实；另一个问题是，在这个（简单的版本控制）系统中，文件名并没有被保存——我们仅保存了文件的内容。**

 上述类型的对象我们称之为 **数据对象（blob object**）。 利用 `git cat-file -t` 命令，可以让 ***Git*** 告诉我们其内部存储的任何对象类型，只要给定该对象的 ***SHA-1*** 值：

```
$ git cat-file -t 372991728e3348b2c32b8dd5377885f2f77173ff
blob
```

