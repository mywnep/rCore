# no_std 与 panic 处理

## 1. 使用 `#![no_std]` 后，为什么需要自己或通过依赖提供 panic handler？

>[!note]- 我的理解
>在 Rust 程序通过显式调用 `panic!` 或数组越界触发 `panic` 时，`#[panic_handler]` 标记的函数决定 `panic` 后该如何处理，​`std` 中提供了该函数的实现，但 `no_std` 则需要自行处理

## 2. 当前 panic handler 中，`#[panic_handler]`、`&PanicInfo`、`-> !` 和 `loop {}` 各有什么作用？发生 panic 后会怎样？

>[!note]- 我的理解
>`#[panic_handler]` 是属性标记，被标记的函数进行 `panic` 后的处理。
>`PanicInfo` 是 `core` 中的结构体，其共享引用 `&PanicInfo` 被传递给 `#[panic_handler]` 标记的函数，提供 `panic` 的相关信息；
>`loop` 是这里的占位实现，会让执行流在这里停留；`-> !` 是指返回类型是
>`! (never type)`，表示该函数永远不会正常返回调用处，`#[panic_handler]` 规定了该函数签名，这里通过 `loop {}` 实现了这一要求，可以改为关机或重启等

## 3. `lang_items.rs` 模块和 Rust 的 lang items 机制有什么区别？修改文件名或函数名，与删除 `#[panic_handler]`，分别会有什么影响？

>[!note]- 我的理解
>Rust 的 lang items 是编译器与库之间的约定，用来识别承担特殊语言功能的函数、类型或 trait。
>大部分的 lang items 由 `core` 提供，`std` 实现了默认的 `panic` 处理实现，这里则需要自己手动实现或通过依赖提供（使用库现成的处理函数）
>这里通过 `#[panic_handler]` 指定 panic 处理实现，而 `lang_items.rs` 只是存放这段代码的普通模块。
>修改文件名，需要同步修改 `main.rs` 中的模块声明；修改处理函数的名称，不影响编译器识别，如果存在直接调用则需要同步修改调用处。如果没有其他 `#[panic_handler]` 函数时，删除 `#[panic_handler]` 会导致没有入口处理 `panic` 无法通过编译
