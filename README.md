# oh-regex-jsonrpc

将 rust 的 regex 封装为 JSONRPC2 stdio 协议，可以对 regex 的各个方法执行测试。

## 测试

### 1. find
输入：
```
{"jsonrpc":"2.0","id":1,"method":"find","params":{"pattern":"\\d+","test_content":"abc123xyz"}}
```

输出：
```
{"jsonrpc":"2.0","id":1,"result":{"matched":true,"result":{"End":6,"Start":3,"Text":"123"}}}
```