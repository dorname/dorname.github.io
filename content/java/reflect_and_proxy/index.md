---
title: java的反射和代理
date: 2024-06-26
extra:
    image: ../java.png
taxonomies:
  tags:
    - build_tools
  authors:
    - liguangqiao
---

# java的反射和代理机制

## java的反射特性

**反射特性**允许程序在运行时检查、修改和操作自身的结构

**反射特性**主要解决java运行时的灵活性和动态性问题，使得Java应用程序能够在运行时加载、探查、使用和修改类和对象的行为。

**使用场景：**

1. 动态类加载。
2. 访问类的内部信息
3. 调用方法和访问字段
4. 数组操作
5. 实现泛型api
6. 动态代理
7. 调试和测试工具

#### 示例1：动态加载`java.util.ArrayList`类，调用其方法

```java
public void test(){
  try {
            Class<?> clazz = Class.forName("java.util.ArrayList");
            Object arr = clazz.getDeclaredConstructor().newInstance();
            Method add = clazz.getMethod("add",Object.class);
            Method size = clazz.getMethod("size");
            Method get = clazz.getMethod("get",int.class);
            add.invoke(arr,"test");
            int len = (Integer) size.invoke(arr);
            String first = (String) get.invoke(arr,0);
            System.out.println("长度："+len);
            System.out.println("第一个元素："+first);
        } catch (ClassNotFoundException | NoSuchMethodException | InstantiationException | IllegalAccessException |
                 InvocationTargetException e) {
            throw new RuntimeException(e);
        }
}
```

#### 示例2：访问类的内部信息

```java
 public void accessAll(){
        Class<?> hashClazz = java.util.HashMap.class;
        
        Method[] methods = hashClazz.getMethods();

        //访问所有方法
        System.out.println("Public Methods:");
        for (Method method : methods) {
            System.out.println(method.getName());
        }

        //访问所有属性
        Field[] fields = hashClazz.getDeclaredFields();
        System.out.println("\nFields (including private):");
        for (Field field : fields){
            System.out.println(field.getName());
        }

        //获取构造器时间
        Constructor[] constructors = hashClazz.getConstructors();
        System.out.println("\nConstructors:");
        for (Constructor constructor: constructors){
            System.out.println(constructor.toString());
        }

    }
```

#### 示例3：修改私有字段的值

```JAVA
 public void privateField(){
//        Class<?> clazz = Class.forName("reflectadproxy.refect.Person"); //这种写法需要考虑异常处理 因为存在类找不到等异常
        Class<?> clazz = Person.class; //这种写法之所以不用考虑异常处理是因为类已经找到了,不通包下的类需要import进来
        try {
            Object person = clazz.getConstructor().newInstance();
            // 访问私有字段
            Field valueField = Person.class.getDeclaredField("name");

            valueField.setAccessible(true);
            String name = (String) valueField.get(person);
            System.out.println("Value Field: " + name);

            Method setName = clazz.getMethod("setName", String.class);

            setName.invoke(person,"测试");

            String newName = (String) valueField.get(person);
            System.out.println("newValue Field: " +newName);

        }catch (NoSuchMethodException | NoSuchFieldException | InstantiationException | IllegalAccessException |
                InvocationTargetException e){

        }

    }
```

#### 示例4：动态代理对方法进行拦截

```JAVA
//计算接口
public interface Calculator {
    int add(int a, int b);
    int subtract(int a, int b);
}
//颜色接口
public interface Color {
    String showColor(String color);
}
//基础类
public class BasicCalculator implements Calculator,Color {
    public int add(int a, int b) {
        return a + b;
    }

    public int subtract(int a, int b) {
        return a - b;
    }

    public String showColor(String color){
        return color;
    }
}
//拦截处理类
import java.lang.reflect.InvocationHandler;
import java.lang.reflect.Method;
public class LoggingHandler implements InvocationHandler {
    private final Object target;

    public LoggingHandler(Object target) {
        this.target = target;
    }

    @Override
    public Object invoke(Object proxy, Method method, Object[] args) throws Throwable {
        System.out.println("Method called: " + method.getName());
        System.out.println("With arguments: ");
        for (Object arg : args) {
            System.out.println(arg);
        }
        return method.invoke(target, args);
    }
}
//代理测试类
import java.lang.reflect.Proxy;

public class ProxyExample {
    public void test(){
        Calculator realCalculator = new BasicCalculator();
        Calculator proxyCalculator = (Calculator) Proxy.newProxyInstance(
                Calculator.class.getClassLoader(),
                new Class<?>[] { Color.class,Calculator.class},
                new LoggingHandler(realCalculator)
        );

        // 使用代理对象执行操作
        int result = proxyCalculator.add(5, 3);
        System.out.println("Result: " + result);

        result = proxyCalculator.subtract(10, 6);
        System.out.println("Result: " + result);
    }

    public void test2(){
        // 创建原始对象
        BasicCalculator realCalculator = new BasicCalculator();

        // 创建动态代理对象，同时代理 Calculator 和 Color 接口
        Object proxy = Proxy.newProxyInstance(
                // 类加载器
                BasicCalculator.class.getClassLoader(),
                // 拦截哪些方法
                new Class<?>[] { Calculator.class, Color.class },
                //拦截哪个实例
                new LoggingHandler(realCalculator)
        );

        // 代理转换为 Calculator 接口
        Calculator proxyCalculator = (Calculator) proxy;
        // 使用代理对象执行计算操作
        int addResult = proxyCalculator.add(5, 3);
        System.out.println("Add Result: " + addResult);
        int subtractResult = proxyCalculator.subtract(10, 6);
        System.out.println("Subtract Result: " + subtractResult);

        // 代理转换为 Color 接口
        Color proxyColor = (Color) proxy;
        // 使用代理对象执行颜色显示
        String colorResult = proxyColor.showColor("Red");
        System.out.println("Input Color is:"+colorResult);
    }
}

```

#### 示例5： 利用反射去进行检测实例中的字段值达到调试的效果

```JAVA
 public static void inspectObject(Object obj) {
        Class<?> clazz = obj.getClass();
        System.out.println("Class Name: " + clazz.getName());

        // List all fields and their values
        Field[] fields = clazz.getDeclaredFields();
        System.out.println("Fields:");
        for (Field field : fields) {
            field.setAccessible(true);
            try {
                System.out.println(field.getName() + " = " + field.get(obj));
            } catch (IllegalAccessException e) {
                System.out.println("Could not access " + field.getName());
            }
        }
    }
```

