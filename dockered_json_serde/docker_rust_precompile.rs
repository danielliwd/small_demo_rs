// 这是dockerfile rust最佳实践之一: 预编译库
// 多阶段build的预编译阶段，把依赖的库都编译好
// 该代码会在Dockerfile中COPY 到 src/main.rs


#[allow(unused_imports)]
#[macro_use]
extern crate serde;

#[allow(unused_imports)]
use serde_json;


fn main() {
}
