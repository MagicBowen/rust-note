- [x]: install rust
- [x]: learn quick start
- [x]: cargo
- [x]: other toolchains： rustdoc, rustc, rustfmt, fls...
- [x]: libraries: std and 3th party
- [x]: libraries: local and src lib
- [x]: test framework
- [x]: doc test framework
- [x]: 3th test framework
- [x]: mock framework
- [x]: embedded env
- [x]: project
- [x]: memory safety and ownership
- [x]: ownership
- [x]: lifecycle
- [x]: coroutine
- [x]: grammar

---

## install

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## quick start

- homesite
https://www.rust-lang.org/

- quick start
https://www.rust-lang.org/learn/get-started

- crates
https://crates.io/

## toolchains

### cargo

- `cargo --version`
- `cargo new project-name`
- build your project with `cargo build`
- run your project with `cargo run`
- test your project with `cargo test`
- build documentation for your project with `cargo doc`
- publish a library to crates.io with `cargo publish`

### rustdoc

- https://doc.rust-lang.org/stable/rustdoc/

### rustup

- install the rust language server for VS CODE

```sh
rustup component add rls
```

- learn rust from doc

```sh
rustup doc
```

### 3th test frameworks

BDD：

- [stainless](https://github.com/reem/stainless)
- [speculate](https://crates.io/crates/speculate)

Mutation Test：

- [mutagen](https://crates.io/crates/mutagen)

Mock:

- [mockiato](https://crates.io/crates/mockiato)

## module system

package -> crate -> module -> path

- package : A Cargo feature that lets you build, test, and share crates, includes a `cargo.toml`
- crate : A tree of modules that produces a library or executable, includes a `main.rs` and/or a `lib.rs`
- module : Let you control the organization, scope, and privacy of paths
- path : A way of naming an item, such as a struct, function, or module

workspace:
- project workspace including several package
- top workspace toml file describing the member packages
- independent publish

## integrate with C and cmake

- https://flames-of-code.netlify.com/blog/rust-and-cmake/

## embeded development

- https://www.rust-lang.org/what/embedded
- https://rust-embedded.github.io/book/
- https://github.com/rust-embedded/awesome-embedded-rust
- https://docs.rust-embedded.org/embedonomicon/preface.html

collection in embeded system:

- https://docs.rust-embedded.org/book/collections/index.html

interact with C:

- https://docs.rust-embedded.org/book/c-tips/index.html

rust with other build system such as cmake:

- https://github.com/rust-embedded/book/issues/61

rust with RTOS:

- https://github.com/rust-embedded/book/issues/62

AKA patterns?

## memory safety

首先，RUST认为”内存泄露“并不属于”内存安全“；

内存安全保证不会出现以下问题：

- 使用未定义内存
- 空指针
- 悬垂指针
- 缓冲区溢出
- 非法释放未分配的指针或已经释放的指针

内存泄露并不在rust的内存安全保证范围内，内存泄露会由于以下原因：

- 线程崩溃，析构函数无法调用
- 使用引用计数造成循环引用
- 调用RUST标准库中的forget函数主动泄露

区分：
- 值语义 与 引用语义； 值语义，复制后是独立的；引用语义是别名；
- 变量绑定的双层含义：1）空间属性；2）时间属性（生命周期）
- 所有权：RUST中变量绑定后，只有一个变量负责对应内存的写与释放
- 拷贝 与 移动；
- 共享所有权 与 独占所有权；

思考：

- 拷贝与移动：靠一定编译器检查的标记，而非编译器自动识别左值和右值调用用户自定义的语义操作；

- 生命周期：靠生命周期标记以及编译器检查

- 内存安全：靠严格的一刀切的规则“可变不共享，共享不可变”，严格的编译器检查（结合生命周期标记），降低了灵活性为代价

- 是否可以将所有权细分：将owner的释放的所有权与写的所有权分开？

- rust默认是move优先，C++默认是copy优先。取决有那个是更常遇到的。

- rust的成员函数是否可变，取决于传入的self的类型；而C++靠的是成员的const修饰（成员函数的const其实修饰的是this指针）；

summary:

- 可变引用是独占引用，不可变引用是可共享引用。独占引用和共享引用的生命周期不能重叠；(共享不可变，可变不共享；但是有例外：cell，refcell是共享引用，但是可变)
- 引用的生命周期分析：引用不能比被引用对象活的更长（非unsafe情况下）；分析依赖于rust自动分析（推导规则）以及生命周期标注；
- 对象在被引用时不能移动（否则之前的引用会不安全）；


reference：
- [知乎关于unsafe的讨论](https://zhuanlan.zhihu.com/p/103353305)
- [知乎对比rust和C++的move](https://www.zhihu.com/question/369738529/answer/1006022667)

## ownership

所有权规则：

- Rust中的值都有一个被称为所有者变量。
- 值只能有一个所有者变量。
- 所有者变量离开作用域，此值将被丢弃。

所有权转移
- 满足copy trait的进行赋值后，是拷贝语义，所以是两个变量，各自有所有权。或者显示调用 clone方法后赋值的。
- 其它move语义的赋值后，老的被move走，所有权也移走了，老的就不能用了；
- 上面两条也适用于函数传参，或者返回值；
- 当所有权变量离开作用域时，会调用drop（有的话）去清除；
- mut引用会转移所有权；可以有多个&mut引用，但是rust会做生命周期分析，重叠期间只有一个有效，其余不能用；

借用：
- 不改变所有权，只是借用；那么需要传递只读引用；
- 借用后只可读，无法修改原有的值；
- 借用和所有权共存期间，rust会做生命周期分析，重叠期间不允许使用借用；


## lifecycle

约束：借用的生命周期不能长于出借方；

保证：
- 编译器自动分析借用方和出借方生命周期（编译期词法作用域）；
- NLL（非词法作用域生命周期）优化：让代码更好些，部分有效；
- 跨函数借用的生命周期约束检查，会让编译器复杂，所以大多数情况下需要开发者标注借用的生命周期；
- 生命周期标注的唯一原则：约束借用者的的生命周期不长于出借者；


生命周期：
- 每个引用都有其生命周期(lifetime)，也就是引用保持有效的作用域；
- 大部分的生命周期是隐含并可以推断的；
- 一些引用的生命周期存在一些不同方式的关联，因此Rust也需要我们使用生命周期的标注来表明他们的关系；
- 生命周期注解描述了多个引用生命周期相互关系，并不影响其生命周期；
- 单个生命周期注解本身没有多少意义，因为生命周期是表示多个引用的生命周期参数的相互关系；
- 在函数签名中指定生命周期参数时，我们没有改变任何传入后返回的值的生命周期，而是指任何不遵循这个约定的传入值将被拒绝；
- 'static是静态生命周期，其存活于整个程序期间。所有的字符串字面量都拥有'static生命周期。因为它们被储存在程序二进制文件中；
- 生命周期注解的本质也是泛型。其代表了任意一个作用域，这个作用域是需要推断与计算的；

函数生命周期标注：
- 函数参数的生命周期叫做输入生命周期（Input lifetime）；返回值生命周期被称为输出生命周期（output lifetime）；
- 函数的生命周期检查是为了检查函数的输出借用生命周期不长于输入借用；
- 禁止没有任何输入的情况下返回引用；

结构体生命周期标注：
- 在结构体有引用类型成员的时候，约定结构体实例的生命周期应短于或等于任意一个引用成员的生命周期；

三处可省略的生命周期标注：
- 每一个引用都有自己的生命周期参数。例如一个引用参数有一个生命周期：fn foo<'a>(x: &'a i32),两个引用参数有两个生命周期：fn foo<'a,'b>(x:&'a i32,y: &'b i32)。
- 如果只有一个输入生命周期，它被用于所有的输出生命周期参数：fn foo <'a>(x: &'a i32) -> &'a i32
- 如果方法有多个输入生命周期参数，如果其中有&self 或者&mut self，并且self的生命周期被赋予所以输出生命周期参数。

思考：

- lifecycle标记法？
- 模板参数与lifecycle标记耦合在一起？

reference：
- [知乎上关于lifetime的讨论](https://www.zhihu.com/question/265324947/answer/965999967)

## coroutine

### concept

- [协程的分类](https://www.zhihu.com/question/23955356/answer/732629313)
- [有栈和无栈协程的优劣对比](https://www.zhihu.com/question/65647171)

- [rust 异步IO](https://zhuanlan.zhihu.com/p/52538218)
- [rust coroutine](https://zhuanlan.zhihu.com/p/97574385)
- [协程介绍](https://blog.csdn.net/hyman_yx/article/details/52251261)
- [理解coroutine](http://anruence.com/2018/01/22/coroutine/)
- [协程对比](https://www.zhihu.com/question/65647171)
- [Kotlin coroutine解析](https://www.jianshu.com/p/2659bbe0df16)
- [C++ 协程的近况、设计与实现中的细节和决策](https://www.jianshu.com/p/837bb161793a)
- [Java为什么没有协程](https://www.zhihu.com/question/332042250/answer/734115120)

summary：

- 有栈协程相当于用户态线程，内存占用多，与真实线程比节省了内核切换开销；
- 无栈协程（await，async），效率更高，内存开销更小；需要一个执行器（executor）；

## tutorials

- [rust book](https://doc.rust-lang.org/book/title-page.html)
- [rust指针理解](https://zhuanlan.zhihu.com/p/76945648)
- [rust引用理解](https://zhuanlan.zhihu.com/p/88926962)
- [rust内存管理理解：区别于C++](https://zhuanlan.zhihu.com/p/74181780)
- [rust的send和sync理解](https://zhuanlan.zhihu.com/p/64699643)
- [rust的closure理解](https://zhuanlan.zhihu.com/p/64417628)
- [rust的协程理解](https://zhuanlan.zhihu.com/p/97574385)
- [rust实现操作系统](https://zhuanlan.zhihu.com/p/53064186)
- [rust实现脚本语言：反思](https://zhuanlan.zhihu.com/p/69237793)
- [rust使用体验](https://zhuanlan.zhihu.com/p/88478551)
- [rust标准库至linked-list](https://zhuanlan.zhihu.com/p/69091784)
- [rust实现双向链表](http://cglab.ca/~abeinges/blah/too-many-lists/book/README.html)