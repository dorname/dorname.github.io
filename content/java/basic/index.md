---
title: java基础
date: 2024-06-26
extra:
    image: ../java.png
taxonomies:
  tags:
    - java
    - code language
  authors:
    - liguangqiao
---

# Java

## 反射基础

```JAVA
 // 动态加载类
  Class<?> clazz = Class.forName("java.util.ArrayList");
```

 `Class` 是 Java 反射 API 中的一个核心类，用于表示所有类和接口在运行时的类型信息。

### **Class<?>**

####  解释 `Class<?>`

1. **`Class` 类**：这是 java.lang.reflect 包中的一个类，表示在运行时的类和接口。每个类都被 `Class` 类的一个实例所表示，无论它是一个普通类、接口、枚举还是注解。
2. **泛型 `<?>`（通配符）**：在 Java 中，`?` 表示通配符，用于泛型编程。`Class<?>` 表示“未知的 `Class` 类型”，它是泛型语法的一部分。使用 `?` 作为类型参数，你可以引用任何类型的 `Class` 对象。这种方式比使用 `Class` 类型更加安全，因为它防止了向 `Class` 对象中放入错误类型的实例。

####  为什么使用 `Class<?>`

- **类型安全**：使用 `Class<?>` 比简单使用非泛型的 `Class` 类型更安全。它表明你并不关心 `Class` 对象的具体类型，但仍然保留了泛型的好处，即确保如果你对这个类对象进行操作，你不会错误地处理类型信息。
- **避免类型警告**：直接使用 `Class` 会导致编译器警告，如“Class 是原生类型”。使用 `Class<?>` 可以避免这些警告，使代码更干净，更符合 Java 的泛型约定。

总结来说，`Class<?>` 是 Java 反射和泛型用法的一个例子，它提供了一种类型安全和灵活的方式来处理在编译时类型未知的类对象

### Class.java的getMethod方法

`getMethod`方法返回一个反映由此`Class`对象表示的类或接口中指定的公共成员方法的`Method`对象。`name`参数是一个字符串，指定所需方法的简单名称。`parameterTypes`参数是一个`Class`对象数组，这些对象按声明的顺序标识方法的形式参数类型。如果`parameterTypes`为`null`，则被视为一个空数组。

如果这个`Class`对象表示一个数组类型，那么此方法会找到数组类型从`Object`类继承的任何公共方法，除了`clone()`方法外。

如果这个`Class`对象代表一个接口，那么这个方法不会找到`Object`中任何隐式声明的方法。因此，如果接口或其任何超接口中没有显式声明任何方法，则此方法不会找到任何方法。

此方法不会找到名称为`"<init>"`或`"<clinit>"`的任何方法。
