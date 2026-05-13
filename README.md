# oh-regex-jsonrpc

将 rust 的 regex 封装为 JSONRPC2 stdio 协议，可以对 regex 的各个方法执行测试。

## 输入输出示例：

输入：
```
{"jsonrpc":"2.0","id":1,"method":"find","params":{"pattern":"\\d+","test_content":"abc123xyz"}}
```

输出：
```
{"jsonrpc":"2.0","id":1,"result":{"matched":true,"result":{"End":6,"Start":3,"Text":"123"},"highlight":"abc<code>123</code>xyz"}}
```

## Methods:
1. find
2. find_iter
3. captures
4. replace
5. replace_all
6. split
7. escape
8. is_match
