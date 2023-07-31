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

修改属性的默认特性：`Object.defineProperty`方法，参数1：对象实例；参数2：属性名称；参数3：描述数据特性的对象

```javascript
var obj = {
    "name": "test"
};
console.log(Object.getOwnPropertyDescriptor(obj, 'name'))
Object.defineProperty(obj,"name",{
    writable:false,
    value:"can't write"
});
console.log(Object.getOwnPropertyDescriptor(obj, 'name'))
console.log("1:",obj.name);
obj.name = "test";
console.log("2:",obj.name);
Object.defineProperty(obj,"name",{
    configurable:false,
    value:"can't delete"
});
console.log(Object.getOwnPropertyDescriptor(obj, 'name'))
delete obj.name;//严格模式下会抛出错误
console.log("3:",obj.name);

//output
/**
{value: 'test', writable: true, enumerable: true, configurable: true}

{value: 'can't write', writable: false, enumerable: true, configurable: true}

1: can't write

2: can't write

{value: 'can't delete', writable: false, enumerable: true, configurable: false}

3: can't delete
*/
```

**注意：一个属性被定义为不可配置之后就不能再变会可配置的属性，再次调用`Object.defineProperty()`去修改非`writable`属性会导致错误**

实际实验中，`wriable`修改同样会导致错误，故不确定是文档问题还是vscode的运行环境问题。

### 访问器属性

访问器属性不包含数据值。相反，它们包含一个获取（`getter`）函数和一个设置（`setter`）函数，不过这两个函数不是必需的。在读取访问器属性时，会调用获取函数，这个函数的责任就是返回一个有效的值。在写入访问器属性时，会调用设置函数并传入新值，这个函数必须决定对数据做出什么修改。访问器属性有 4 个特性描述它们的行为

- `[[Configurable]]`:表示属性是否可以通过`delete`删除并重新定义，是否可以修改它的特性，以及是否可以把它改为数据属性。默认情况下，所有直接定义在对象上的属性的这个特性都是`true`。
- `[[Enumberable]]`:表示属性是否可以通过`for-in`循环返回。默认情况下对象上的属性的这个特性都是`true`。
- `[[Get]]`:获取函数，在读取属性时调用。默认值为`undefined`。
- `[[Set]]`:设置函数，在写入属性时调用。默认值为`undefined`。

访问器属性是不能直接定义的，必须使用`Object.defineProperty()`

```javascript
let age = {
    fakeAge:27,
    realAge_:28
};
Object.defineProperty(age,"realAge",{
    get(){
        return this.realAge_;
    },
    set(newValue){
        this.realAge_ = newValue;
        this.fakeAge = this.realAge_-1;
    }
});
age.realAge = 29;
console.log(age,age.realAge,age.fakeAge);

//output
//{fakeAge: 28, realAge_: 29, realAge: <accessor>} 29 28
```

**注意：**获取函数和设置函数不一定都要定义。

- 只定义获取函数意味着属性是只读的，尝试修改属性会被忽略。在严格模式下，尝试写入只定义了获取函数的属性会抛出错误。

```javascript
let age = {
    fakeAge:27,
    realAge_:28
};
Object.defineProperty(age,"realAge",{
    get(){
        return this.realAge_;
    },
    // set(newValue){
    //     this.realAge_ = newValue;
    //     this.fakeAge = this.realAge_-1;
    // }
});
age.realAge = 29;
console.log(age.realAge,age.fakeAge);

//output
//28 27
```

- 类似地，只有一个设置函数的属性是不能读取的，非严格模式下读取会返回 undefined，严格模式下会抛出错误。

```javascript
let age = {
    fakeAge:27,
    realAge_:28
};
Object.defineProperty(age,"realAge",{
    // get(){
    //     return this.realAge_;
    // },
    set(newValue){
        this.realAge_ = newValue;
        this.fakeAge = this.realAge_-1;
    }
});
age.realAge = 29;
console.log(age.realAge,age.fakeAge);
//undefined 28
```

