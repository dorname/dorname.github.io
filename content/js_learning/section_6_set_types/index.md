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

### 目标 

​	定型数组（Typed Arrays）是为了解决在JavaScript中进行二进制数据处理时所遇到的一些问题而引入的。JavaScript原生的数组虽然非常强大，但在处理二进制数据方面存在一些限制和性能问题。这些问题包括：

1. **数据类型问题：** JavaScript中的普通数组是动态类型的，一个数组可以包含不同类型的数据。但在处理二进制数据时，需要确保数据类型的一致性，以避免解释错误的二进制值。
2. **性能问题：** 对于大量的二进制数据，JavaScript数组的处理效率相对较低。因为普通数组是动态的，包含了很多附加信息，而在处理二进制数据时，我们希望能够更紧凑地存储数据。
3. **内存问题：** 普通数组在存储数字时占用较多的内存，因为它们需要支持动态类型和其他功能。对于大型的二进制数据，这可能导致内存占用问题。

​	定型数组通过引入固定数据类型和更紧凑的内存表示，解决了这些问题。它们提供了一种高性能的方式来操作和处理二进制数据，特别适用于需要进行低级别的二进制计算、图像处理、音视频编解码等任务。通过定型数组，开发人员可以更直接地访问和操作二进制数据，从而提高性能并减少内存占用。

**注意：**

1. `ArrayBuffer`分配的内存不能超过`Number.MAX_SAFE_INTEGER`

   即
   $$
   2^{53}-1
   $$

2. 声明`ArrayBuffer`则会将所有二进制位初始化  为0。 

3. 声明`ArrayBuffer`的堆内存可以被当成垃圾回收，不用手动释放。

​	实际上我们无法直接对`ArrayBuffer`进行内容的读写操作。要读写必须通过视图进行操作。

### 视图

#### DataView

`DataView `可以让你以各种不同的数据类型和字节顺序来读取和写入 `ArrayBuffer` 中的数据，从而更灵活地处理二进制数据。

`DataView` 适用于以下情况：

1. **跨数据类型操作：** 如果需要在同一个 ArrayBuffer 中以不同的数据类型进行读写操作，`DataView` 更适合，因为它可以手动指定数据类型。
2. **精确控制字节顺序：** `DataView` 允许你指定字节顺序，这在处理与不同机器端序（Big Endian 或 Little Endian）有关的数据时非常有用。
3. **处理定制格式的二进制数据：** 对于一些特殊格式的二进制数据，如网络通信协议、文件格式等，`DataView` 更容易进行灵活的解析和构建。

`DataView`的主要方法：

- **创建DataView**

   使用 `new DataView(buffer, byteOffset, byteLength)` 来创建一个 DataView。`buffer` 是一个 ArrayBuffer 对象，`byteOffset` 是从哪个字节开始，`byteLength` 是要处理的字节数。

  ```js
  const buf = new ArrayBuffer(16); 
  // DataView 默认使用整个 ArrayBuffer 
  const fullDataView = new DataView(buf); 
  console.log(fullDataView.byteOffset); // 0 
  console.log(fullDataView.byteLength); // 16 
  console.log(fullDataView.buffer === buf); // true
  ```

  ```js
  const buf = new ArrayBuffer(16); 
  // DataView 默认使用整个 ArrayBuffer 
  const fullDataView = new DataView(buf,1); 
  console.log(fullDataView.byteOffset); // 1
  console.log(fullDataView.byteLength); // 15 
  console.log(fullDataView.buffer === buf); // true
  ```

  ```js
  const buf = new ArrayBuffer(16); 
  // DataView 默认使用整个 ArrayBuffer 
  const fullDataView = new DataView(buf,1,4); 
  console.log(fullDataView.byteOffset); // 1
  console.log(fullDataView.byteLength); // 4 
  console.log(fullDataView.buffer === buf); // true
  ```

  1. 元素类型

  | 元素类型 | 字节 | 说明               | 等价的C类型    | 值范围                       |
  | -------- | ---- | ------------------ | -------------- | ---------------------------- |
  | Int8     | 1    | 8位有符号整数      | signed char    | -128~127                     |
  | Uint8    | 1    | 8位无符号整数      | unsigned char  | 0~255                        |
  | Int16    | 2    | 16位有符号整数     | short          | -32768~32767                 |
  | Uint16   | 2    | 16位无符号整数     | unsigned short | 0~65535                      |
  | Int32    | 4    | 32位有符号整数     | int            | -2 147 483 648~2 147 483 647 |
  | Uint32   | 4    | 32位无符号整数     | unsigned int   | 0~4 294 967 295              |
  | Float32  | 4    | 32位IEEE-754浮点数 | float          | -3.4e+38~+3.4e+38            |
  | Float64  | 8    | 64位IEEE-754浮点数 | double         | -1.7e+308~+1.7e+308          |

- **读写数据**

  `DataView` 为上表中的每种类型都暴露了 `get` 和` set` 方法，这些方法使用 `byteOffset`（字节偏移量）定位要读取或写入值的位置。

  ```js
  
  const buf = new ArrayBuffer(2); 
  const view = new DataView(buf); 
  // 说明整个缓冲确实所有二进制位都是 0 
  // 检查第一个和第二个字符
  console.log(view.getInt8(0)); // 0 
  console.log(view.getInt8(1)); // 0  
  //使用 setUint8() 方法分别将 buf 中第一个字节和第二个字节设置为 255。在这里，setUint8(0, 255) 和 setUint8(1, 0xFF) 是等效的，因为 0xFF 也是 255 的十进制表示。
  // 检查整个缓冲
  console.log(view.getInt16(0)); // 0 使用 view.getInt16(0) 尝试读取第一个字节和第二个字节合并成的 16 位整数。
  view.setUint8(0,255);//1111111100000000
  console.log(view.getInt16(0)); // -256
  view.setUint8(1,0xFF);//11111111 11111111
  console.log(view.getInt16(0)); // -1
  ```

- **字节顺序**

  “字节序”指的是计算系统维护的一种字节顺序的约定。`DataView` 只支持两种约定：大端字节序和小端字节序。`JavaScript` 运行时所在系统的原生字节序决定了如何读取或写入字节，但 `DataView` 并不遵守这个约定。对一段内存而言，`DataView` 是一个中立接口，它会遵循你指定的字节序。`DataView` 的所有` API `方法都以大端字节序作为默认值，但接收一个可选的布尔值参数，设置为` true` 即可启用小端字节序。

  - 大端字节序也称为“网络字节序”，意思是最高有效位保存在第一个字节，而最低有效位保存在最后一个字节。
  - 小端字节序正好相反，即最低有效位保存在第一个字节，最高有效位保存在最后一个字节。

  ```js
  // 在内存中分配两个字节并声明一个 DataView 
  const buf = new ArrayBuffer(2); 
  const view = new DataView(buf); 
  // 填充缓冲，让第一位和最后一位都是 1 
  view.setUint8(0, 0x80); // 设置最左边的位等于 1 
  view.setUint8(1, 0x01); // 设置最右边的位等于 1 
  // 缓冲内容（为方便阅读，人为加了空格）
  // 0x8 0x0 0x0 0x1 
  // 1000 0000 0000 0001 
  // 按大端字节序读取 Uint16 
  // 0x80 是高字节，0x01 是低字节
  // 0x8001 = 2^15 + 2^0 = 32768 + 1 = 32769 
  console.log(view.getUint16(0)); // 32769 
  // 按小端字节序读取 Uint16 
  // 0x01 是高字节，0x80 是低字节
  // 0x0180 = 2^8 + 2^7 = 256 + 128 = 384 
  console.log(view.getUint16(0, true)); // 384 
  // 按大端字节序写入 Uint16 
  view.setUint16(0, 0x0004); 
  // 缓冲内容（为方便阅读，人为加了空格）
  // 0x0 0x0 0x0 0x4 
  // 0000 0000 0000 0100 
  console.log(view.getUint8(0)); // 0 
  console.log(view.getUint8(1)); // 4 
  // 按小端字节序写入 Uint16 
  view.setUint16(0, 0x0002, true); 
  // 缓冲内容（为方便阅读，人为加了空格）
  // 0x0 0x2 0x0 0x0 
  // 0000 0010 0000 0000 
  console.log(view.getUint8(0)); // 2 
  console.log(view.getUint8(1)); // 0
  ```

- **边界情形**

  `DataView` 完成读、写操作的前提是必须有充足的缓冲区，否则就会抛出 `RangeError`：

  ```js
  const buf = new ArrayBuffer(6); 
  const view = new DataView(buf); 
  // 尝试读取部分超出缓冲范围的值
  view.getInt32(4); 
  // RangeError 
  // 尝试读取超出缓冲范围的值
  view.getInt32(8); 
  // RangeError 
  // 尝试读取超出缓冲范围的值
  view.getInt32(-1); 
  // RangeError 
  // 尝试写入超出缓冲范围的值
  view.setInt32(4, 123); 
  // RangeError
  ```

  `DataView` 在写入缓冲里会尽最大努力把一个值转换为适当的类型，后备为 0。如果无法转换，则抛出错误：

  ```js
  const buf = new ArrayBuffer(1); 
  const view = new DataView(buf); 
  view.setInt8(0, 1.5); 
  alert(view.getInt8(0)); // 1 
  view.setInt8(0, [4]); 
  alert(view.getInt8(0)); // 4 
  view.setInt8(0, 'f'); 
  alert(view.getInt8(0)); // 0 
  view.setInt8(0, Symbol()); 
  // TypeError
  ```

### 定型数组

JavaScript提供了以下几种类型的定型数组：

1. `Int8Array`, `Uint8Array`, `Uint8ClampedArray`: 8位整数数组，分别表示带符号整数、无符号整数和用于图像处理的无符号整数（值范围在0-255之间）。

2. `Int16Array`, `Uint16Array`: 16位整数数组，分别表示带符号整数和无符号整数。

3. `Int32Array`, `Uint32Array`: 32位整数数组，分别表示带符号整数和无符号整数。

4. `Float32Array`, `Float64Array`: 32位和64位浮点数数组，分别表示单精度和双精度浮点数。

5. 如果定型数组没有用任何值初始化，则其关联的缓冲会以 0 填充：

   ```js
   const ints = new Int32Array(4); 
   alert(ints[0]); // 0 
   alert(ints[1]); // 0 
   alert(ints[2]); // 0 
   alert(ints[3]); // 0
   ```

定型数组的构造函数和实例都有一个` BYTES_PER_ELEMENT` 属性，返回该类型数组中每个元素的大小:

```js
console.log(Int16Array.BYTES_PER_ELEMENT); // 2 
console.log(Int32Array.BYTES_PER_ELEMENT); // 4 
const ints = new Int32Array(1), 
floats = new Float64Array(1); 
console.log(ints.BYTES_PER_ELEMENT); // 4 
console.log(floats.BYTES_PER_ELEMENT); // 8
```

定型数组方法基本与普通数组一致。其中需要注意的是：

- 返回新数组的方法也会返回包含同样元素类型（element type）的新定型数组：

  ```js
  const ints = new Int16Array([1, 2, 3]); 
  const doubleints = ints.map(x => 2*x); 
  console.log(doubleints instanceof Int16Array); // true
  ```

- 定型数组有一个 `Symbol.iterator` 符号属性，因此可以通过 `for..of` 循环和扩展操作符来操作：

  ```js
  const ints = new Int16Array([1, 2, 3]); 
  for (const int of ints) { 
   console.log(int); 
  } 
  // 1 
  // 2 
  // 3 
  console.log(Math.max(...ints)); // 3
  ```

- 定型数组同样使用数组缓冲来存储数据，而数组缓冲无法调整大小。因此，下列方法不适用于定型数组：

  1. `concat`
  2. `pop`
  3. `push`
  4. `shift`
  5. `splice`
  6. `unshift`

- 定型数组也提供了两个新方法，可以快速向外或向内复制数据：

  1. `set`

     ```js
     // 创建长度为 8 的 int16 数组
     const container = new Int16Array(8); 
     // 把定型数组复制为前 4 个值
     // 偏移量默认为索引 0 
     container.set(Int8Array.of(1, 2, 3, 4)); 
     console.log(container); // [1,2,3,4,0,0,0,0] 
     // 把普通数组复制为后 4 个值
     // 偏移量 4 表示从索引 4 开始插入
     container.set([5,6,7,8], 4); 
     console.log(container); // [1,2,3,4,5,6,7,8] 
     // 溢出会抛出错误
     container.set([5,6,7,8], 7); 
     // RangeError
     ```

  2. `subarray`

     ```js
     const source = Int16Array.of(2, 4, 6, 8); 
     // 把整个数组复制为一个同类型的新数组
     const fullCopy = source.subarray(); 
     console.log(fullCopy); // [2, 4, 6, 8] 
     // 从索引 2 开始复制数组
     const halfCopy = source.subarray(2); 
     console.log(halfCopy); // [6, 8] 
     // 从索引 1 开始复制到索引 3 
     const partialCopy = source.subarray(1, 3); 
     console.log(partialCopy); // [4, 6]
     ```

- 定型数组没有原生的拼接能力，但使用定型数组 API 提供的很多工具可以手动构建：

  ```js
  // 第一个参数是应该返回的数组类型 
  // 其余参数是应该拼接在一起的定型数组
  function typedArrayConcat(typedArrayConstructor, ...typedArrays) { 
   // 计算所有数组中包含的元素总数
   const numElements = typedArrays.reduce((x,y) => (x.length || x) + y.length); 
   // 按照提供的类型创建一个数组，为所有元素留出空间
   const resultArray = new typedArrayConstructor(numElements); 
   // 依次转移数组
   let currentOffset = 0; 
   typedArrays.map(x => { 
   resultArray.set(x, currentOffset); 
   currentOffset += x.length; 
   }); 
   return resultArray; 
  } 
  const concatArray = typedArrayConcat(Int32Array, 
   Int8Array.of(1, 2, 3), 
   Int16Array.of(4, 5, 6), 
   Float32Array.of(7, 8, 9)); 
  console.log(concatArray); // [1, 2, 3, 4, 5, 6, 7, 8, 9] 
  console.log(concatArray instanceof Int32Array); // true
  ```

- **下溢和上溢**

  定型数组中值的下溢和上溢不会影响到其他索引，但仍然需要考虑数组的元素应该是什么类型。定型数组对于可以存储的每个索引只接受一个相关位，而不考虑它们对实际数值的影响。以下代码演示了如何处理下溢和上溢：

  ```js
  // 长度为 2 的有符号整数数组
  // 每个索引保存一个二补数形式的有符号整数
  // 范围是-128（-1 * 2^7）~127（2^7 - 1）
  const ints = new Int8Array(2); 
  // 长度为 2 的无符号整数数组
  // 每个索引保存一个无符号整数
  // 范围是 0~255（2^7 - 1）
  const unsignedInts = new Uint8Array(2); 
  // 上溢的位不会影响相邻索引
  // 索引只取最低有效位上的 8 位
  unsignedInts[1] = 256; // 0x100 
  console.log(unsignedInts); // [0, 0] 
  unsignedInts[1] = 511; // 0x1FF 
  console.log(unsignedInts); // [0, 255] 
  // 下溢的位会被转换为其无符号的等价值
  // 0xFF 是以二补数形式表示的-1（截取到 8 位）, 
  // 但 255 是一个无符号整数
  unsignedInts[1] = -1 // 0xFF (truncated to 8 bits) 
  console.log(unsignedInts); // [0, 255] 
  // 上溢自动变成二补数形式
  // 0x80 是无符号整数的 128，是二补数形式的-128 
  ints[1] = 128; // 0x80 
  console.log(ints); // [0, -128] 
  // 下溢自动变成二补数形式
  // 0xFF 是无符号整数的 255，是二补数形式的-1 
  ints[1] = 255; // 0xFF 
  console.log(ints); // [0, -1]
  ```

- 除了 8 种元素类型，还有一种“夹板”数组类型：`Uint8ClampedArray`，不允许任何方向溢出。超出最大值 255 的值会被向下舍入为 255，而小于最小值 0 的值会被向上舍入为 0。

定型数组的简单例子

```js
// 创建一个大小为 16 字节的 ArrayBuffer
const buffer = new ArrayBuffer(16);

// 创建一个 Int32Array 视图，表示整个 ArrayBuffer
const intArray = new Int32Array(buffer);

// 在 Int32Array 中设置值
intArray[0] = 42;
intArray[3] = 36;
console.log(intArray);//Int32Array(4) [42, 0, 0, 36, buffer: ArrayBuffer(16), byteLength: 16, byteOffset: 0, length: 4]


// 创建一个 Uint8Array 视图，表示 ArrayBuffer 中的一部分
const byteView = new Uint8Array(buffer); // 从第 8 个字节开始，长度为 4 字节

// 在 Uint8Array 中查看数据
console.log(byteView); // Uint8Array(16) [42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, buffer: ArrayBuffer(16), byteLength: 16, byteOffset: 0, length: 16]
```

- `Int32Array`：每个成员占据 4 个字节，所以在一个 16 字节的 `ArrayBuffer` 中，只能容纳 16 / 4 = 4 个 `Int32` 类型的成员。
- `Uint8Array`：每个成员占据 1 个字节，所以在一个 16 字节的 `ArrayBuffer` 中，可以容纳 16 个 `Uint8` 类型的成员。

## ***Map***

**ES6新特性：**键值对类型***Map***

1. 创建映射***Map***，`size`属性代表映射大小

   ```js
   let m = new Map();
   // m = 1;
   let arr = [1,4,5,6];
   let m1 = new Map([
       ['a',1],
       ['b',2]
   ]);
   let m2 = new Map(arr.entries());
   console.log(m);
   console.log(m1);
   console.log(m2);
   //Map(0) {size: 0}
   //Map(2) {size: 2, a => 1, b => 2} 
   //Map(4) {size: 4, 0 => 1, 1 => 4, 2 => 5, 3 => 6}
   
   
   ```

2. `set`方法

   **注意：map类型的键、值可以是任意js数据类型，而object只能是数字、字符串和下划线**

   ```javascript
   m.set(m1,m2);
   let a = function(){
       console.log(1)
   };
   m.set(function(){
       console.log(1)
   },arr);
   m.set(a,arr);
   console.log(m);
   //Map(3) {size: 3, Map(2) {…} => Map(4) {…}, ƒ () => (4) [1, 4, 5, 6, …], ƒ () => (4) [1, 4, 5, 6, …]}
   
   ```

3. `get`取值方法

   ```js
   
   console.log(m.get(m1));
   console.log(m.get(function(){
       console.log(1)
   }));
   console.log(m.get(a));
   
   //Map(4) {size: 4, 0 => 1, 1 => 4, 2 => 5, 3 => 6}
   
   //undefined
   
   //(4) [1, 4, 5, 6]
   ```

   **注意：键或者值的内部属性、属性值发生变化时，不会影响原本的映射关系例如：**

   ```js
   arr = [7,8,9];
   console.log(m.get(a));
   m1.delete('a');
   console.log(m1,m.get(m1));
   //(4) [1, 4, 5, 6]
   
   //Map(1) {size: 1, b => 2} Map(4) {size: 4, 0 => 1, 1 => 4, 2 => 5, 3 => 6}
   ```

4. `delete`删除方法。传入对应的键，删除指定键值对。

5. `clear`清空方法。清空`map`中所有的键值对。

   **注意：Map 内部使用 SameValueZero 比较操作（ECMAScript 规范内部定义，语言中不能使用），基本上相当于使用严格对象相等的标准来检查键的匹配性，SameValueZero 比较也可能导致意想不到的冲突：**

   ```js
   const m = new Map(); 
   const a = 0/"", // NaN 
    b = 0/"", // NaN 
    pz = +0, 
    nz = -0; 
   alert(a === b); // false 
   alert(pz === nz); // true 
   m.set(a, "foo"); 
   m.set(pz, "bar"); 
   alert(m.get(b)); // foo 
   alert(m.get(nz)); // bar
   ```

### **顺序与迭代**

`Map`实例与`Object`实例主要差异是：

`Map`实例会维护键值对插入顺序，因此可以根据插入顺序执行迭代操作。

`Map`同样拥有三个迭代器`keys`、`values`、`entries`，顺序与插入顺序保持一致。

### **Map vs Object**

1. **内存占用：**

   `Object` 和` Map` 的工程级实现在不同浏览器间存在明显差异，但存储单个键/值对所占用的内存数量都会随键的数量线性增加。批量添加或删除键/值对则取决于各浏览器对该类型内存分配的工程实现。不同浏览器的情况不同，但给定固定大小的内存，`Map` 大约可以比 `Object` 多存储 50%的键/值对

2. **插入性能：**

   `Object`和`Map`中插入新键/值对的消耗大致相当，不过插入 Map 在所有浏览器中一般会稍微快一点儿。对这两个类型来说，插入速度并不会随着键/值对数量而线性增加。如果代码涉及大量插入操作，那么显然 Map 的性能更佳。

3. **查找速度：**

   与插入不同，从大型`Object` 和 `Map` 中查找键/值对的性能差异极小，但如果只包含少量键/值对，则 `Object` 有时候速度更快。在把 `Object` 当成数组使用的情况下（比如使用连续整数作为属性），浏览器引擎可以进行优化，在内存中使用更高效的布局。这对 `Map` 来说是不可能的。对这两个类型而言，查找速度不会随着键/值对数量增加而线性增加。如果代码涉及大量查找操作，那么某些情况下可能选择 `Object` 更好一些。

4. **删除性能：**

   使用 `delete` 删除 `Object` 属性的性能一直以来饱受诟病，目前在很多浏览器中仍然如此。为此，出现了一些伪删除对象属性的操作，包括把属性值设置为 `undefined` 或 `null`。但很多时候，这都是一种讨厌的或不适宜的折中。而对大多数浏览器引擎来说，`Map` 的 `delete()`操作都比插入和查找更快。如果代码涉及大量删除操作，那么毫无疑问应该选择 `Map`。

**综上所述：**

大部分应用场景下可以优先使用`Map`类型，但作为长期支持的原始类型`Object`可以用于底层应用开发。

## ***WeakMap***

**注意：弱映射中的键只能是`Object`或者继承自`Object`的子类（`null`除外）尝试使用非对象设置键会抛出`TypeError`。值的类型没有限制。**

```js
const key1 = {
    id:1
},key2 = {
    id:2
},key3 = {
    id:3
};
const wm = new WeakMap([
    [key1,{
        test_fst:1
    }],
    [key2,{
        test_sec:2
    }]
]);

console.log(wm);
//WeakMap {{id: 2} => {test_sec: 2}, {id: 1} => {test_fst: 1}}
console.log(wm.get(key1));
//{test_fst: 1}

wm.set(key3,{
    test_trd:3
});
console.log(wm);
//WeakMap {{id: 2} => {test_sec: 2}, {id: 3} => {test_trd: 3}, {…} => {test_fst: 1}}

console.log(wm.has(key3));
//true

console.log(wm.get(key3));
//{test_trd: 3}

wm.delete(key3);
console.log(wm.has(key3));
//false

console.log(wm.get(key3));
//undefined
```

`WeakMap` 中`weak`表示弱映射的键是“弱弱地拿着”的。意思就是，这些键不属于正式的引用，不会阻止垃圾回收。进一步理解就是说，如果一个对象作为 `WeakMap` 的键，而且没有其他的引用指向这个对象，那么在下一次垃圾回收时，这个键值对将会被自动移除，从而释放内存。

**注意：`WeakMap`不能遍历，方法同 `get`、`set`、`has`、`delete`**

综上`Map`和`WeakMap`的区别：

- `Map`和`WeakMap`本质上都是键值对集合，前者的键和值都没有限制，而后者的键必须是`object`类型或者继承自`object`的类型其中又除去`null`类型
- `WeakMap`的键不会阻止键对象被垃圾回收，`Map`的键会阻止键对象被垃圾回收。
- `WeakMap`不支持迭代方法，`Map`支持迭代方法。

### 弱映射的应用

1. 私有变量

   ```js
   const wm = new WeakMap(); 
   class User { 
    constructor(id) { 
    this.idProperty = Symbol('id'); 
    this.setId(id); 
    } 
    setPrivate(property, value) { 
    const privateMembers = wm.get(this) || {}; 
    privateMembers[property] = value; 
    wm.set(this, privateMembers); 
    } 
    getPrivate(property) { 
    return wm.get(this)[property]; 
    } 
    setId(id) { 
    this.setPrivate(this.idProperty, id); 
    } 
    getId() { 
    return this.getPrivate(this.idProperty); 
    } 
   } 
   const user = new User(123); 
   console.log(user.getId()); // 123 
   user.setId(456); 
   console.log(user.getId()); // 456 
   // 并不是真正私有的
   console.log(wm.get(user)[user.idProperty]); // 456
   ```

   对于上面的实现，外部代码只需要拿到对象实例的引用和弱映射，就可以取得“私有”变量了。为了避免这种访问，可以用一个闭包把 WeakMap 包装起来，这样就可以把弱映射与外界完全隔离开了：

   ```js
   const User = (() => {
       const wm = new WeakMap();
       class User {
           constructor(id) {
               this.idProperty = Symbol('id');
               this.setId(id);
           }
           setPrivate(property, value) {
               const privateMembers = wm.get(this) || {};
               privateMembers[property] = value;
               wm.set(this, privateMembers);
           }
           getPrivate(property) {
               return wm.get(this)[property];
           }
           setId(id) {
               this.setPrivate(this.idProperty, id);
           }
           getId(id) {
               return this.getPrivate(this.idProperty);
           }
       }
       return User;
   })();
   const user = new User(123);
   console.log(user.getId()); // 123 
   user.setId(456);
   console.log(user.getId()); // 456
   ```

   

2. **dom**节点元数据

   因为` WeakMap` 实例不会妨碍垃圾回收，所以非常适合保存关联元数据。来看下面这个例子，其中使用了常规的 `Map`：

   ```js
   const m = new Map(); 
   
   const loginButton = document.querySelector('#login'); 
   
   // 给这个节点关联一些元数据
   
   m.set(loginButton, {disabled: true}); 
   ```

   假设在上面的代码执行后，页面被 `JavaScript` 改变了，原来的登录按钮从 `DOM `树中被删掉了。但由于映射中还保存着按钮的引用，所以对应的 `DOM` 节点仍然会逗留在内存中，除非明确将其从映射中

   删除或者等到映射本身被销毁。如果这里使用的是弱映射，如以下代码所示，那么当节点从` DOM` 树中被删除后，垃圾回收程序就

   可以立即释放其内存（假设没有其他地方引用这个对象）：

   ```js
   const wm = new WeakMap(); 
   
   const loginButton = document.querySelector('#login'); 
   // 给这个节点关联一些元数据
   wm.set(loginButton, {disabled: true}); 
   ```

## ***Set***

**ECMAScript 6**新增的` Set` 是一种新集合类型，为这门语言带来集合数据结构。`Set` 在很多方面都像是加强的` Map`，这是因为它们的大多数 **API** 和行为都是共有的。`Set `可以包含任何 JavaScript 数据类型作为值

```js
const s = new Set();
let obj_zero = {
    id:0
},obj_fst = {
    id:1
};
s.add(obj_zero).add(obj_fst).add({
    id:3
});
console.log(s);
//Set(3) {size: 3, {id: 0}, {id: 1}, {id: 3}}
console.log("size:",s.size);
//size: 3
console.log("Has obj_first: ",s.has(obj_fst));
//Has obj_first:  true
s.delete({
    id:3
});
console.log("After delete: ",s);
//After delete:  Set(3) {size: 3, {id: 0}, {id: 1}, {id: 3}}

var third;
for(let i of s.keys()){
    console.log(i)
    third = i;
}
//{id: 0}
//{id: 1}
//{id: 3}

s.delete(third);
console.log("After delete: ",s);
//After delete:  Set(2) {size: 2, {id: 0}, {id: 1}}

s.clear();
console.log("After clear: ",s);
//After clear:  Set(0) {size: 0}

```

### **迭代方法**

迭代器：`keys`、`values`、`entries`。

迭代方法：`forEach`

1. `forEach`

   ```js
   s.forEach(function(val,duplicate_val,set){
       console.log(val,duplicate_val,set);
   });
   //{id: 0} {id: 0} Set(2) {size: 2, {id: 0}, {id: 1}}
   //{id: 1} {id: 1} Set(2) {size: 2, {id: 0}, {id: 1}}
   ```

2. `keys`

   ```js
   const keys = s.keys();
   console.log("keys:",keys);//keys: SetIterator {{id: 0}, {id: 1}}
   for(let key of keys){
       console.log("key:",key);
       //key: {id: 0}
       //key: {id: 1}
   }
   console.log("keys after used: ",keys);//keys after used:  SetIterator
   
   let keys_arr = Array.from(s.keys());
   console.log("keys_arr: ",keys_arr); //keys_arr:  (2) [{id:0}, {id:1}]
   ```

   **注意：一个迭代器不可重复使用**

3. `values`

   ```js
   const values = s.values();
   console.log("values:",values);//values: SetIterator {{id: 0}, {id: 1}}
   
   for(let val of values){
       console.log("val:",val);
       //val: {id: 0}
       //val: {id: 1}
   }
   console.log("values after used: ",keys);//values after used:  SetIterator
   
   let values_arr = Array.from(s.values());
   console.log("values_arr: ",values_arr); // //values_arr:  (2) [{id:0}, {id:1}]
   ```

4. `entries`

   ```js
   const entries = s.entries();
   console.log("entries:",entries);//entries: SetIterator {{id: 0} => {id: 0}, {id: 1} => {id: 1}}
   
   for(let entry of entries){
       console.log("entry:",entry);
       //entry: (2) [{id:0}, {id:0}]
       //entry: (2) [{id:1}, {id:1}]
   }
   console.log("entries after used: ",entries);//entries after used:  SetIterator
   
   let entries_arr = Array.from(s.entries());
   console.log("entries_arr: ",entries_arr); //entries_arr:  (2) [[{id:0}, {id:0}], [{id:1}, {id:1}]]
   ```

## ***WeakSet***

同`WeakMap`，`WeakSet`具有以下特点：

1. 成员都是对象（引用）；
2. 成员都是弱引用，随时可以消失，不阻止垃圾回收。可以用来保存DOM 节点，不容易造成内存泄露；
3. 不能遍历，方法有 add、delete、has；

## 迭代与扩展操作

`...`为迭代数据类型的扩展操作符。