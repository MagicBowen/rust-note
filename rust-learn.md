- [x]: install rust
- [x]: learn quick start
- [x]: cargo
- [x]: other toolchains： rustdoc, rustc, rustfmt, fls...
- [x]: libraries: std and 3th party
- [x]: libraries: local and src lib
- [x]: test framework
- [x]: doc test framework
- [ ]: 3th test framework
- [ ]: mock framework
- [x]: embedded env
- [x]: project
- [x]: memory safety and ownership
- [ ]: lifecycle
- [ ]: coroutine
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

- 拷贝与移动：靠一定编译器检查的标记，而非编译器自动识别左值和右值调用用户自定义的语义操作

- 生命周期：靠生命周期标记以及编译器检查

- 内存安全：靠严格的一刀切的规则“可变不共享，共享不可变”，严格的编译器检查（结合生命周期标记），降低了灵活性为代价

- rust默认是move优先，C++默认是copy优先。取决有那个是更常遇到的。

- rust的成员函数是否可变，取决于传入的self的类型；而C++靠的是成员的const修饰（成员函数的const其实修饰的是this指针）；

summary:

- 可变引用是独占引用，不可变引用是可共享引用。独占引用和共享引用的生命周期不能重叠；(共享不可变，可变不共享；但是有例外：cell，refcell是共享引用，但是可变)
- 引用的生命周期分析：引用不能比被引用对象活的更长（非unsafe情况下）；分析依赖于rust自动分析（推导规则）以及生命周期标注；
- 对象在被引用时不能移动（否则之前的引用会不安全）；

## lifecycle

- lifecycle标记法？
- 模板参数与lifecycle标记耦合在一起？

## coroutine

- [rust 异步IO](https://zhuanlan.zhihu.com/p/52538218)
- [rust coroutine](https://zhuanlan.zhihu.com/p/97574385)
- [协程介绍](https://blog.csdn.net/hyman_yx/article/details/52251261)
- [理解coroutine](http://anruence.com/2018/01/22/coroutine/)
- [协程对比](https://www.zhihu.com/question/65647171)
- [Kotlin coroutine解析](https://www.jianshu.com/p/2659bbe0df16)
- [C++ 协程的近况、设计与实现中的细节和决策](https://www.jianshu.com/p/837bb161793a)

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


---

## summary

- cargo : how to pass compile args to crates?
- embeded: how to integrate with c? rust invokes c and c invokes rust?
- lifecycle summary; differences with C++?
- memory model differences with C++?
- rust project : build, test, mock ...
- 协程： stackless，stackful；协程与并发...
