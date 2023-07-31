---
title: js学习笔记(8)——对象、类与面向对象编程
date: 2023-07-31
extra:
    image: ../js_logo.png
taxonomies:
  tags:
    - js
  authors:
    - liguangqiao  
---

# **对象、类与面向对象编程**

## 理解对象

创建自定义对象的通常方式是创建**Object**的一个新实例：

```javascript
let person = new Object(); 
person.name = "Nicholas"; 
person.age = 29; 
person.job = "Software Engineer"; 
person.sayName = function() { 
 console.log(this.name); 
};

//前面的例子
let person = { 
name: "Nicholas", 
age: 29, 
job: "Software Engineer", 
sayName() { 
console.log(this.name); 
} 
};
```

## 属性的类型

### 数据属性

- `[[Configurable]]`:表示属性是否可以通过`delete`删除并重新定义，是否可以修改它的特性,以及是否可以把它改为访问器属性。注：默认所有直接定义在对象的属性的这个特性都是true。
- `[[Enumberable]]`:表示属性是否可以通过`for-in`循环返回。默认情况下，所有直接定义在对象上的属性的这个特性都是true。
- `[[Writable]]`:表示属性的值是否可以被修改。默认情况下，所以直接定义在对象上的属性的，这个特性都是true。
- `[[Value]]`:包含属性实际的值。这就是前面提到的那个读取和写入属性值的位置。这个特性的默认值为`undefined`。

查看对象属性的方式：`Object.getOwnPropertyDescriptor`方法

```javascript
var obj = {
    "name":"test"
};
console.log(Object.getOwnPropertyDescriptor(obj, 'name'))

//output
//{ value: 'test', writable: true, enumerable: true, configurable: true }
```

