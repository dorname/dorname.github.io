---
title: js学习笔记(6)——集合类型
date: 2023-08-12
extra:
    image: ../js_logo.png
taxonomies:
  tags:
    - js
  authors:
    - liguangqiao  
---

# **集合引用类型**

## ***Object***

`Object`很适合存储和在应用程序间交换数据。

```js
//声明方式
let obj = new Object();
obj.age = 1;
let obj_one = {
    age:1
};
//属性访问
//点语法
console.log(obj.age);
//变量语法
console.log(obj["age"]);
```

`Object`不管是实验和生产环境中都很常用所以这里不做过多的阐述

## ***Array***

`Array`同样是`js`中常用的引用类型。

`js`的数组不同于其他编程语言，其每一个槽位都可以存储任意类型的数据。

### 数组创建

```js
//长度为20的空数组
let colors = new Array(20);
//成员为"1","2"的数组
colors = new Array("1","2");
colors = ["1","2"];
```

**ES6中还有两个创建数组的方法**

- 通过`from`方法去创建数组

```js
// 字符串会被拆分为单字符数组
console.log(Array.from("Matt")); // ["M", "a", "t", "t"]

// 可以使用 from()将集合和映射转换为一个新数组
const m = new Map().set(1, 2) 
 .set(3, 4); 
const s = new Set().add(1) 
 .add(2) 
 .add(3) 
 .add(4); 
console.log(Array.from(m)); // [[1, 2], [3, 4]] 
console.log(Array.from(s)); // [1, 2, 3, 4]

// Array.from()对现有数组执行浅复制
const a1 = [1, 2, 3, 4]; 
const a2 = Array.from(a1); 
console.log(a1); // [1, 2, 3, 4] 
alert(a1 === a2); // false

// 可以使用任何可迭代对象
const iter = { 
 *[Symbol.iterator]() { 
 yield 1; 
 yield 2; 
 yield 3; 
 yield 4; 
 } 
}; 
console.log(Array.from(iter)); // [1, 2, 3, 4]

// arguments 对象可以被轻松地转换为数组
function getArgsArray() { 
 return Array.from(arguments); 
} 
console.log(getArgsArray(1, 2, 3, 4)); // [1, 2, 3, 4] 
// from()也能转换带有必要属性的自定义对象
const arrayLikeObject = { 
 0: 1, 
 1: 2, 
 2: 3, 
 3: 4, 
 length: 4 
}; 
console.log(Array.from(arrayLikeObject)); // [1, 2, 3, 4]
```

- `Array.from`还有两个参数，

  第二个参数是可选的映射函数，用于增强新数组的值

  ```js
  Array.from([1,2,3,4],(item)=>item+1);//[2,3,4,5]
  //等价于
  Array.from([1,2,3,4]).map((item)=>item+1);//[2,3,4,5]
  ```

  第三个参数指定映射函数中的上下文this，固定作用域和调用者

  ```js
  Array.from([1,2,3,4],function(item){
      console.log(this);
      return item+this.num;
  },{id:"test",num:2});
  ```

- 通过`of`方法去创建数组

  ```js
  /*Array.of()可以把一组参数转换为数组。这个方法用于替代在 ES6之前常用的 Array.prototype. 
  slice.call(arguments)，一种异常笨拙的将 arguments 对象转换为数组的写法：*/
  console.log(Array.of(1, 2, 3, 4)); // [1, 2, 3, 4] 
  console.log(Array.of(undefined)); // [undefined]
  ```

### 数组空位

生产环境数组空位会有诸多隐患故暂时不做过多的了解。

### 数组索引

常规的数组索引与其他语言保持一致，本小节重点将放在数组的`length`属性上。

- `length`并非只读属性，可修改

- 通过修改`length`属性可新增和删除数组元素。

  - 新增

    直接增加数组长度，会新增多个空的`undefined`元素

    ```js
    let colors = ["r","g","b"];
    colors.length = 4;
    console.log(colors[3]); //undefined
    ```

    对`length`位置进行赋值

    ```js
    let colors = ["r","g","b"];
    colors[3] = "y";
    console.log(colors[3]); //"y"
    ```

  - 删除

    ```js
    let colors = ["r","g","b"];
    colors.length = 2;
    console.log(colors[2]); // undefined
    ```

### 数组检测

- 单页应用

  使用`instanceof`进行数组类型的判断

  ```js
  console.log([] instanceof Array); //true
  ```

- 多页应用

  使用`Array.isArray()`方法进行数组类型判断

  ```js
  Array.isArray([]);
  ```

### 迭代器方法

**ES6**中Array的原型暴露了3个数组检索方法

- `keys()`

- `values()`
- `entries()`

```js
const a = ["foo", "bar", "baz", "qux"]; 
// 因为这些方法都返回迭代器，所以可以将它们的内容
// 通过 Array.from()直接转换为数组实例
const aKeys = Array.from(a.keys()); 
const aValues = Array.from(a.values()); 
const aEntries = Array.from(a.entries()); 
console.log(aKeys); // [0, 1, 2, 3] 
console.log(aValues); // ["foo", "bar", "baz", "qux"] 
console.log(aEntries); // [[0, "foo"], [1, "bar"], [2, "baz"], [3, "qux"]]
```

使用 **ES6** 的解构可以非常容易地在循环中拆分键/值对：

```js
const a = ["foo", "bar", "baz", "qux"]; 
for (const [idx, element] of a.entries()) { 
 console.log(idx); 
 console.log(element); 
} 
// 0 
// foo 
// 1 
// bar 
// 2 
// baz 
// 3 
// qux
```

### 复制和填充方法

**ES6** 新增了两个方法：批量复制方法 `copyWithin()`，以及填充数组方法 `fill()`。这两个方法的函数签名类似，都需要指定既有数组实例上的一个范围，包含开始索引，不包含结束索引。使用这个方法不会改变数组的大小。

- `fill()`拥有三个参数，参数1填充值、参数2开始索引和参数3结束索引

  使用 `fill()`方法可以向一个已有的数组中插入全部或部分相同的值。开始索引用于指定开始填充的位置，它是可选的。如果不提供结束索引，则一直填充到数组末尾。负值索引从数组末尾开始计算。也可以将负索引想象成数组长度加上它得到的一个正索引：

  ```js
  const zeroes = [0, 0, 0, 0, 0]; 
  // 用 5 填充整个数组
  zeroes.fill(5); 
  console.log(zeroes); // [5, 5, 5, 5, 5] 
  zeroes.fill(0); // 重置
  // 用 6 填充索引大于等于 3 的元素
  zeroes.fill(6, 3); 
  console.log(zeroes); // [0, 0, 0, 6, 6] 
  zeroes.fill(0); // 重置
  // 用 7 填充索引大于等于 1 且小于 3 的元素
  zeroes.fill(7, 1, 3); 
  console.log(zeroes); // [0, 7, 7, 0, 0]; 
  zeroes.fill(0); // 重置
  // 用 8 填充索引大于等于 1 且小于 4 的元素
  // (-4 + zeroes.length = 1) 
  // (-1 + zeroes.length = 4) 
  zeroes.fill(8, -4, -1); 
  console.log(zeroes); // [0, 8, 8, 8, 0]; 
  fill()静默忽略超出数组边界、零长度及方向相反的索引范围：
  const zeroes = [0, 0, 0, 0, 0]; 
  // 索引过低，忽略
  // (-10 + zeroes.length = -5) 
  // (-6 + zeroes.length = -1) 
  zeroes.fill(1, -10, -6); 
  console.log(zeroes); // [0, 0, 0, 0, 0] 
  // 索引过高，忽略
  // (10 > zeroes.length) 
  // (15 > zeroes.length) 
  zeroes.fill(1, 10, 15); 
  console.log(zeroes); // [0, 0, 0, 0, 0] 
  // 索引反向，忽略
  zeroes.fill(2, 4, 2); 
  console.log(zeroes); // [0, 0, 0, 0, 0] 
  // 索引部分可用，填充可用部分
  zeroes.fill(4, 3, 10) 
  console.log(zeroes); // [0, 0, 0, 4, 4]
  ```

- `copyWithin()`

  与 `fill()`不同，`copyWithin()`会按照指定范围浅复制数组中的部分内容，然后将它们插入到指定索引开始的位置。开始索引和结束索引则与 `fill()`使用同样的计算方法：

  ```js
  let ints, 
   reset = () => ints = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; 
  reset(); 
  // 从 ints 中复制索引 0 开始的内容，插入到索引 5 开始的位置
  // 在源索引或目标索引到达数组边界时停止
  ints.copyWithin(5); 
  console.log(ints); // [0, 1, 2, 3, 4, 0, 1, 2, 3, 4] 
  reset(); 
  // 从 ints 中复制索引 5 开始的内容，插入到索引 0 开始的位置
  ints.copyWithin(0, 5); 
  console.log(ints); // [5, 6, 7, 8, 9, 5, 6, 7, 8, 9]
  reset();
  // 从 ints 中复制索引 0 开始到索引 3 结束的内容
  // 插入到索引 4 开始的位置
  ints.copyWithin(4, 0, 3); 
  alert(ints); // [0, 1, 2, 3, 0, 1, 2, 7, 8, 9] 
  reset(); 
  // JavaScript 引擎在插值前会完整复制范围内的值
  // 因此复制期间不存在重写的风险
  ints.copyWithin(2, 0, 6); 
  alert(ints); // [0, 1, 0, 1, 2, 3, 4, 5, 8, 9] 
  reset(); 
  // 支持负索引值，与 fill()相对于数组末尾计算正向索引的过程是一样的
  ints.copyWithin(-4, -7, -3); 
  alert(ints); // [0, 1, 2, 3, 4, 5, 3, 4, 5, 6] 
  copyWithin()静默忽略超出数组边界、零长度及方向相反的索引范围：
  let ints, 
   reset = () => ints = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; 
  reset(); 
  // 索引过低，忽略
  ints.copyWithin(1, -15, -12); 
  alert(ints); // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; 
  reset() 
  // 索引过高，忽略
  ints.copyWithin(1, 12, 15); 
  alert(ints); // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; 
  reset(); 
  // 索引反向，忽略
  ints.copyWithin(2, 4, 2); 
  alert(ints); // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; 
  reset(); 
  // 索引部分可用，复制、填充可用部分
  ints.copyWithin(4, 7, 10) 
  alert(ints); // [0, 1, 2, 3, 7, 8, 9, 7, 8, 9];
  ```

### 转换方法

所有对象都会有三个转换方法：

- `toLocaleString`

  返回一个逗号分隔的数组值的字符串。它与另外两个方法唯一的区别是，为了得到最终的字符串，会调用数组每个值的 `toLocaleString()`方法，而不是`toString()`方法。

  ```js
  let person1 = { 
   toLocaleString() { 
   return "Nikolaos"; 
   }, 
   toString() { 
   return "Nicholas"; 
   } 
  }; 
  let person2 = { 
   toLocaleString() { 
   return "Grigorios"; 
   }, 
   toString() { 
   return "Greg"; 
   } 
  }; 
  let people = [person1, person2]; 
  alert(people); // Nicholas,Greg 
  alert(people.toString()); // Nicholas,Greg 
  alert(people.toLocaleString()); // Nikolaos,Grigorios
  ```

  这里定义了两个对象` person1` 和` person2`，它们都定义了 `toString()`和 `toLocaleString()`方法，而且返回不同的值。然后又创建了一个包含这两个对象的数组 `people`。在将数组传给 `alert()`时，输出的是`"Nicholas,Greg"`，这是因为会在数组每一项上调用 toString()方法（与下一行显式调用`toString()`方法结果一样）。而在调用数组的 `toLocaleString()`方法时，结果变成了`"Nikolaos, Grigorios"`，这是因为调用了数组每一项的 `toLocaleString()`方法。

- `toString` 返回由数组中每个值的等效字符串拼接而成的一个逗号分隔的字符串。也就是说，对数组的每个值都会调用其 `toString()`方法，以得到最终的字符串。

- `valueOf` 返回的还是数组本身。

  继承的方法 `toLocaleString()`以及 `toString()`都返回数组值的逗号分隔的字符串。如果想使用不同的分隔符，则可以使用 `join()`方法。`join()`方法接收一个参数，即字符串分隔符，返回包含所有项的字符串。来看下面的例子：

  ```js
  let colors = ["red", "green", "blue"]; 
  
  alert(colors.join(",")); // red,green,blue 
  
  alert(colors.join("||")); // red||green||blue 
  ```

  这里在 `colors` 数组上调用了 `join()`方法，得到了与调`toString()`方法相同的结果。传入逗号，结果就是逗号分隔的字符串。最后一行给 `join()` 传入了双竖线，得到了字符串`"red||green||blue"`。如果不给 `join()`传入任何参数，或者传入 `undefined`，则仍然使用逗号作为分隔符。

**注意：如果数组中某一项是 `null` 或` undefined`，则在 `join()` 、`toLocaleString()`、`toString()`和` valueOf()`返回的结果中会以空字符串表示 。**

### 栈方法

**LIFO(Last-In-First-Out)**

- `push`： 数组末尾插入一个新的数据项（入栈方法）。
- `pop`：函数数组最后一项（出栈方法）。

### 队列方法

**FIFO(First-In-First-Out)**

- `shift` 获取数组的第一项数据项，并将其移出数组。
- `unshift` 在数组添加任意多的值，然后返回新的数组长度。

### 排序方法

- `reverse` 反向排序

- `sort` 排序结果默认为字符串升序。可以传入自定义的排序策略函数。比较函数就是要返回小于 0、0 和大于 0 的数值，因此减法操作完全可以满足要求。

  ```js
  function compare(value1, value2) { 
   if (value1 < value2) { 
   return -1; 
   } else if (value1 > value2) { 
   return 1; 
   } else { 
   return 0; 
   } 
  }
  let values = [0, 1, 5, 10, 15]; 
  values.sort(compare); 
  console.log(values); // 0,1,5,10,15
  ```

**注意：reverse和sort都返回调用它们的数组引用**

### 操作方法

- `concat`

  `concat`方法可以在现有数组全部元素基础上创建一个新数组。

  1. 首先创建数组副本。
  2. 方法参数添加到数组副本末尾。
  3. 返回副本数组。

  ```js
  let colors = ["red", "green", "blue"]; 
  let colors2 = colors.concat("yellow", ["black", "brown"]); 
  console.log(colors); // ["red", "green","blue"] 
  console.log(colors2); // ["red", "green", "blue", "yellow", "black", "brown"]
  ```

  打平数组参数的行为可以重写，方法是在参数数组上指定一个特殊的符号：`Symbol.isConcatSpreadable`。这个符号能够阻止 `concat()`打平参数数组。相反，把这个值设置为 true 可以强制打平类数组对象：

  ```js
  let colors = ["red", "green", "blue"]; 
  let newColors = ["black", "brown"]; 
  let moreNewColors = { 
   [Symbol.isConcatSpreadable]: true, 
   length: 2, 
   0: "pink", 
   1: "cyan" 
  }; 
  newColors[Symbol.isConcatSpreadable] = false; 
  // 强制不打平数组
  let colors2 = colors.concat("yellow", newColors); 
  // 强制打平类数组对象
  let colors3 = colors.concat(moreNewColors); 
  console.log(colors); // ["red", "green", "blue"] 
  console.log(colors2); // ["red", "green", "blue", "yellow", ["black", "brown"]] 
  console.log(colors3); // ["red", "green", "blue", "pink", "cyan"]
  ```

- `slice`

  `slice`方法用于创建一个包含原有数组中一个或者多个元素的新数组。`slice`方法可以接受一个或者两个参数：返回元素的开始索引和结束索引。如果只有一个参数，则`slice`会返回该索引到数组末尾的所有元素。如果有两个参数，则`slice`返回从开始索引到结束索引对应的所有元素，其中不包含结束索引对应的元素。此操作不会影响原始数组。

  ```js
  let colors = ["red", "green", "blue", "yellow", "purple"]; 
  let colors2 = colors.slice(1); 
  let colors3 = colors.slice(1, 4); 
  let colors4 = colors.slice(1,-1);
  console.log(colors2); // green,blue,yellow,purple 
  console.log(colors3); // green,blue,yellow
  console.log(colors4); // green,blue,yellow
  ```

  **注意：如果`slice()`的参数有负值，那么就以数值长度加上这个负值的结果确定位置。比如，在包含 5 个元素的数组上调用 `slice(-2,-1)`，就相当于调用 `slice(3,4)`。如果结束位置小于开始位置，则返回空数组。**

- `splice`

  `splice`的主要目的是在数组中间插入元素。

  1. 删除。需要给`splice`传2个参数：要删除的第一个元素的位置和要删除的元素数量。

  2. 插入。需要给`splice`传3个参数：开始位置、要删除的元素数量和要插入的元素，可以在数组中指定的位置插入元素。第三个参数之后还可以传第四个、第五个参数，乃至任意多个要插入的元素。

     比如：`splice(2,0,"red","green")`会从数组位置2开始插入字符串`"red"`和`"green"`。

  3. 替换。`splice()`在删除元素的同时可以在指定位置插入新元素，同样要传入 3 个参数：开始位置、要删除元素的数量和要插入的任意多个元素。要插入的元素数量不一定跟删除的元素数量一致。比如，`splice(2, 1, "red", "green")`会在位置 2 删除一个元素，然后从该位置开始向数组中插入`"red"`和`"green"`。

     ```js
     let colors = ["red", "green", "blue", "yellow", "purple"]; 
     colors.splice(2,1,"black","gray");
     console.log(colors,"<<<");
     //(6) ['red', 'green', 'black', 'gray', 'yellow', 'purple'] <<<
     ```

  **注意：`splice()`方法始终返回这样一个数组，它包含从数组中被删除的元素（如果没有删除元素，则返回空数组）。**

  ```js
  let colors = ["red", "green", "blue"]; 
  let removed = colors.splice(0,1); // 删除第一项
  console.log(colors); // green,blue 
  console.log(removed); // red，只有一个元素的数组
  removed = colors.splice(1, 0, "yellow", "orange"); // 在位置 1 插入两个元素
  console.log(colors); // green,yellow,orange,blue 
  console.log(removed); // 空数组
  removed = colors.splice(1, 1, "red", "purple"); // 插入两个值，删除一个元素
  console.log(colors); // green,red,purple,orange,blue 
  console.log(removed); // yellow，只有一个元素的数组
  ```

### 搜索和位置方法

**ES提供两类搜索数组方法：**

- 严格相等搜索，以下方法都接受两个参数：要查找的元素和一个可选的起始搜索位置。

  1. `indexOf`

  2. `lastIndexOf`

     `indexOf()`和 `lastIndexOf()`都返回要查找的元素在数组中的位置，如果没找到则返回-1。

  3. `includes`(**ES7**新增方法)

     返回布尔值，表示是否至少找到一个与指定元素匹配的项。在比较第一个参数跟数组每一项时，会使用全等（===）比较，也就是说两项必须严格相等

  ```js
  let numbers = [1, 2, 3, 4, 5, 4, 3, 2, 1]; 
  alert(numbers.indexOf(4)); // 3 
  alert(numbers.lastIndexOf(4)); // 5 
  alert(numbers.includes(4)); // true 
  alert(numbers.indexOf(4, 4)); // 5 
  alert(numbers.lastIndexOf(4, 4)); // 3 
  alert(numbers.includes(4, 7)); // false 
  let person = { name: "Nicholas" }; 
  let people = [{ name: "Nicholas" }]; 
  let morePeople = [person]; 
  alert(people.indexOf(person)); // -1 
  alert(morePeople.indexOf(person)); // 0 
  alert(people.includes(person)); // false 
  alert(morePeople.includes(person)); // true
  ```

- 按断言函数搜索

  ```js
  const people = [ 
      { 
      name: "Matt", 
      age: 27 
      }, 
      { 
      name: "Nicholas", 
      age: 29 
      } 
     ]; 
     console.log(people.find((element, index, array) => element.age < 28)); 
     // {name: "Matt", age: 27} 
     console.log(people.findIndex((element, index, array) => element.age < 28)); 
     // 0
  
     console.log(people.find((element, index, array) => element.age == 28));
     // undefined
     console.log(people.findIndex((element, index, array) => element.age == 28)); 
     // -1
  ```

  

### 迭代方法

**ES**为数组定义了5个迭代方法。每个方法接受两个参数：以每一项为参数运行的函数、可选的作为函数运行上下文的作用域对象（调整函数中的`this`指向）。传递每个方法的函数接受3个参数：数组元素、元素索引和数组本身。

- `every`

  对数组每一项都运行传入的函数，如果对每一项函数都返回 `true`，则这个方法返回 `true`。

- `filter`

  对数组每一项都运行传入的函数，函数返回 `true` 的项会组成数组之后返回。

- `forEach`

  对数组每一项都运行传入的函数，没有返回值。

- `map`

  对数组每一项都运行传入的函数，返回由每次函数调用的结果构成的数组。

- `some`

  对数组每一项都运行传入的函数，如果有一项函数返回 `true`，则这个方法返回 `true`。这些方法都不改变调用它们的数组。

### 归并方法

​	**ES** 为数组提供了两个归并方法：`reduce()`和 `reduceRight()`。这两个方法都会迭代数组的所有项，并在此基础上构建一个最终返回值。`reduce()`方法从数组第一项开始遍历到最后一项。而 `reduceRight()`从最后一项开始遍历至第一项。

​	这两个方法都接收两个参数：对每一项都会运行的归并函数，以及可选的以之为归并起点的初始值。传给 `reduce()`和 `reduceRight()`的函数接收 4 个参数：上一个归并值、当前项、当前项的索引和数组本身。这个函数返回的任何值都会作为下一次调用同一个函数的第一个参数。如果没有给这两个方法传入可选的第二个参数（作为归并起点值），则第一次迭代将从数组的第二项开始，因此传给归并函数的第一个参数是数组的第一项，第二个参数是数组的第二项。

​	可以使用 `reduce()`函数执行累加数组中所有数值的操作，比如：

```js
let values = [1, 2, 3, 4, 5]; 
let sum = values.reduce((prev, cur, index, array) => prev + cur); 
alert(sum); // 15
```

第一次执行归并函数时，`prev` 是 1，`cur` 是 2。第二次执行时，`prev` 是 3（1 + 2），`cur` 是 3（数组第三项）。如此递进，直到把所有项都遍历一次，最后返回归并结果。

`reduceRight()`方法与之类似，只是方向相反。来看下面的例子：

```js
let values = [1, 2, 3, 4, 5]; 
let sum = values.reduceRight(function(prev, cur, index, array){ 
 return prev + cur; 
}); 
alert(sum); // 15
```

## 定型数组

TODO

## ***Map***

TODO

## ***WeakMap***

TODO

## ***Set***

TODO

## ***WeakSet***

TODO

## 迭代与扩展操作

TODO