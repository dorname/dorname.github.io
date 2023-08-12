---
title: 内存泄漏专题
date: 2023-05-13
extra:
    image: leak.jpg
taxonomies:
  tags:
    - Memory leak
  authors:
    - liguangqiao
---

# 内存泄漏

##  什么是内存泄漏？

内存泄漏指的是在计算机程序中动态分配的内存空间在不再被使用时没有被正确释放或回收的情况。当发生内存泄漏时，程序持续占用着一部分内存，而这些内存无法被其他部分或其他程序使用，从而导致系统的可用内存逐渐减少。

内存泄漏可能由以下情况引起：

1. 动态内存分配未释放：在程序中使用诸如malloc、new等函数或操作符分配内存空间时，如果没有及时调用对应的释放函数或操作符，就会导致内存泄漏。例如，在C++中使用new操作符分配对象的内存，如果没有使用delete操作符释放内存，就会发生内存泄漏。
2. 数据结构中的循环引用：在某些情况下，数据结构中的对象之间可能存在循环引用关系，即对象A引用对象B，对象B又引用对象A，而且这些对象都不再被程序使用。这种情况下，即使这些对象不再需要，由于彼此之间存在引用关系，它们无法被垃圾回收器正确回收，从而导致内存泄漏。
3. 文件或资源未关闭：在使用文件、网络连接、数据库连接等外部资源时，如果没有正确关闭或释放这些资源，就会导致内存泄漏。即使程序不再使用这些资源，但操作系统或底层库可能仍然保持着相关的内存空间。

内存泄漏会导致程序运行时占用的内存不断增加，最终可能导致系统的内存耗尽或程序性能下降。为了避免内存泄漏，开发者应当注意及时释放动态分配的内存，处理对象之间的引用关系，以及正确关闭文件和其他资源。使用合适的内存管理技术和工具，例如自动垃圾回收（Garbage Collection）或智能指针，也可以帮助减少内存泄漏的风险。

## js中的内存泄漏

在JavaScript中，内存泄漏是指无用的对象仍然被占用内存，而无法被垃圾回收器释放的情况。JavaScript中的内存泄漏通常是由以下情况引起的：

1. 未及时清理定时器和事件监听器：在JavaScript中使用定时器（setTimeout或setInterval）或事件监听器（addEventListener）时，如果没有及时清理它们，即使它们不再需要，它们仍然保留对回调函数的引用，导致相关对象无法被垃圾回收。
2. 循环引用：如果存在对象之间的循环引用，即对象A引用了对象B，而对象B又引用了对象A，并且没有其他部分引用这些对象，就会发生内存泄漏。即使这些对象已经不再被程序使用，但由于它们之间的引用关系，垃圾回收器无法正确回收它们所占用的内存。
3. 未释放的DOM元素引用：在JavaScript中，如果在页面中创建了DOM元素的引用，但在后续不再需要时未及时释放，就会发生内存泄漏。即使从页面中删除了这些元素，它们仍然被引用，无法被垃圾回收器回收。
4. 闭包引用：JavaScript中的闭包函数可以访问其外部作用域的变量，如果在闭包函数中引用了外部作用域的变量，而闭包函数本身又没有被及时释放，就会导致引用的变量无法被垃圾回收。
5. 未释放的资源：在JavaScript中涉及到外部资源的操作，如网络请求、数据库连接等，如果没有正确释放这些资源，就会导致内存泄漏。这些资源可能占用系统内存或浏览器资源，而没有被及时释放。

为了避免JavaScript中的内存泄漏问题，开发者可以采取以下措施：

- 及时清理定时器和事件监听器，确保不再需要时将其移除。
- 避免循环引用的产生，特别是在对象之间相互引用时要小心处理。
- 在不再需要时释放对DOM元素的引用，尽量避免保留过多不必要的引用。
- 在使用闭包函数时，注意及时释放不再需要的闭包函数和外部作用域的变量。
- 对涉及到外部资源的操作，如网络请求、数据库连接等，要确保及时关闭或释放这些资源。
- 使用现代的开发工具和浏览器提供的开发者工具进行内存泄漏的检测和分析，以及优化代码和内存使用。
