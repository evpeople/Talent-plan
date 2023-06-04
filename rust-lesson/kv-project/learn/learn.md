# 学到的内容
## project-1
cargo add 通过 -F 启用相关的特性
Rust 项目，需要写doc，可以通过`#![deny(missing_docs)]`，强迫每个pub函数均添加doc
并且在lib.rs的顶部使用这个cfg，那么在所有文件中，均适用
`cargo doc --open`可以在浏览器打开开发所用的所有包的文档
在提交代码前，使用`cargo fmt`和`cargo clippy`，格式化，检查代码。

clap 的使用，`structopt`已经融入到了`clap3.0`中。
## project-2
failure 是rust代码中很重要的一环，我们有一个failure的crate，用于相关的内容

**用一个库之前，首先先注意这个库是不是还在维护**


### Failure Trait
本来是用于代替`Error`的，但是现在已经不再维护了，应该采用AnyHow,或thiserror
Use thiserror if you care about designing your own dedicated error type(s) so that the caller receives exactly the information that you choose in the event of failure. 
This most often applies to library-like code. 
Use Anyhow if you don't care what error type your functions return, you just want it to be easy. 
This is common in application-like code.


#### Fail
Fail 的trait是用于替换`std::error::Error`,代码中的每个error type都需要实现Fail，可以手动实现或者`derive`实现出来。
**这个替代已经失败了**

##### Cause API
Cause API是用于传递Error的发生的原因，并且如果我们需要的话，可以在`downcast_ref`，得到底层的Error
```rust

#![allow(unused_variables)]
fn main() {
while let Some(cause) = fail.cause() {

    if let Some(err) = cause.downcast_ref::<io::Error>() {
        // treat io::Error specially
    } else {
        // fallback case
    }

    fail = cause;
}
}
```
##### Backtraces
backtraces，用于打印调用栈，比起专门的backtraces的 crate，Failure有着一些显著的优点。
##### Context
添加错误的上下文

### Error的四种实践
#### Strings as Error
原型阶段使用，
#### A Custom Fail Type
当我们需要抛出一个并不是来自依赖的Error的时候，可以使用这种方案

#### Using the Error Type
单函数可能抛出多种不同类型的Error的时候，使用这个方式，本质是抛出`Box<Error>`

#### An Error and ErrorKind pair
最稳健的方式，

### Serde

### Bitcask
我现在的实现应该不是最高效的，在Get的时候，不是直接Seek到指定位置，而是按行号读取，这部分可能会是一个性能的优化处，但也不一定。考虑到现在原型还没实现完，就先不要优化了。