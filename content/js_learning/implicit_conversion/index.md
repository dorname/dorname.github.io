---
title: js隐式转换
date: 2023-08-22
extra:
    image: ../js_logo.png
taxonomies:
  tags:
    - js
  authors:
    - liguangqiao  
---

# **js的隐式转换**

## 目前常见的隐式转换

- 相关数据类型

```js
let str = "hello";
let str_num = "1"
let boo = true;
let num = 10;
```

- **隐式数据类型转换**

  1. **隐式转换优先其他类型转换成字符串。**

     数值、布尔值与字符串相加会隐式转换成字符串

     ```js
     console.log(str+boo)//字符串+bool值 "hellotrue"
     console.log(str+num)//字符串+数值 "hello10"
     console.log(str_num+num)//数值型字符串+数值 "110" 
     ```

  2. **非字符串下优先数值。**

     布尔值会隐式转换

     `true` => `1`

     `false` => `0`

     ```js
     console.log(num+10) //bool值+数值 11
     ```

  3. **字符串和布尔值比较：** 当一个字符串和一个布尔值进行比较时，JavaScript 会将布尔值转换为数字（`true` 为 1，`false` 为 0），然后与字符串进行比较。
  
     ```js
     console.log("10">true); //true
     console.log("a">true); //false
     ```
  
  4. **不同数据类型的比较：** 在比较不同数据类型时，JavaScript 会尝试将其中一个值转换为另一个值的数据类型，然后再进行比较。这可能导致一些奇怪的结果。
  
     ```js
     console.log("42"==42);//true
     console.log("42"===42); //false
     ```
  
     **注意：在 JavaScript 中，双等号 `==` 和三等号 `===` 是用于比较值的操作符，核心区别是比较是否做隐式转换：**
  
     - 双等号：双等号进行比较时会进行隐式类型转换。它会尝试将比较的两个值转换为相同的数据类型，然后再进行比较。这就导致了一些令人困惑的情况，因为不同数据类型的值在隐式转换后可能会变得相等。
     - 三等号：三等号进行比较时不会进行隐式类型转换，而是直接比较值和数据类型。只有在值和数据类型都相同的情况下，才会返回 `true`。
