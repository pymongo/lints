enum A {
    A([u8; 255]),
    A2([u8; 255]),
    // 浪费的内存: 255-3
    // 如果 B 只会出现 1% 冷代码，调用次数少的，所以浪费可以忽略
    B([u8; 3])
}
