var mod
var imports = {
    // 共给Rust wasm模块调用的函数
    logit: () => {
        console.log('This was invoked by Rust, writen in JS');
    },
    // hello函数的ptr参数是一个数字，代表WebAssembly.Memory内存数据中的索引
    hello: (ptr, len) => {
        var buf = new Uint8Array(
            mod.instance.exports.memory.buffer, ptr, len
        )
        var msg = new TextDecoder('utf8').decode(buf)
        alert(msg)
    }
}

fetch('http://localhost:7777/hello_wasm.wasm')
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, {
        env: imports
    }))
    .then(module => {
        mod = module
        module.instance.exports.add_one(76)
    })