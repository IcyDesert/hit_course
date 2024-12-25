# HITSZ 抢课小脚本
## YinMo19

请注意！可能没用
---

可以在 release 选择你的系统，下载的压缩包里面有两个可执行文件，分别是
```
hit_course
req_auth
```

hit_course
---
这两个分别是提前准备好课程信息和cookie，直接对应请求，而第二个是直接从头开始走一遍验证。

使用 hit_course 需要提前准备两个文件
```
must.json
pe.json
``` 
可以直接在浏览器中打开 f12 ，选中网络，然后点击必修课程，能够看到一条请求。将这条请求复制位 cURL
![browser](static/QQ_1734983191878.png)
然后将命令粘贴到命令行，追加 ` > must.json`，例如
![](static/QQ_1734983307628.png)
就可以写入一个json文件了。`pe.json` 也是一样的，将别的课程（例如跨专业/体育课）的课程使用同样的方法将 json 导入 `pe.json` ，那么就可以以相同的方法选课。

接下来复制你的cookie，然后使用
```
./hit_course --cookie <your cookie>
```
就可以开始选择你要抢的课了。

需要注意的是，推荐在抢课前提前将导入 json 的步骤做好，否则在抢课的时候你可能甚至没法登陆看到这些课程的内容。

req_auth
---
这个使用会简单一些，但是对于鉴权死掉的系统就没啥用了。直接在命令行输入 
```
./req_auth -h
```
就能看到帮助文档，简单易用。

自己构建
---
使用 rust 编写，在 linux 系统上可能需要安装 openssl 依赖。关于如何安装 rust 环境这里不多说，构建方法为

```
cargo build --release
cargo build --example req_auth --release
```
如果用不了了，可以考虑去源码中更改对应的请求体，因为本身请求体非常丑陋（全都是不明所以的拼音缩写），我看不懂所以基本上就是替换关键词。
