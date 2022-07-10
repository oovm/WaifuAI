## 配置

首次使用需要配置

在同目录下新建 `nai.toml`.

![](https://s2.loli.net/2022/10/12/aK9TLsnduAx3Elj.png)

里面放你的 key.


```toml
[nai]
bearer = "XXXXX"
```

![](https://s2.loli.net/2022/10/12/iuzPAJm4KTnpvWh.png)

打开 `https://novelai.net/`, 然后命令行输入下列代码得到 key

```js
console.log(JSON.parse(localStorage.session).auth_token)
```

![](https://s2.loli.net/2022/10/12/2N3zJaTe659VWsp.png)




