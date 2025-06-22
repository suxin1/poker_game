### 构建项目

Web 开发环境需要在项目目录中执行命令，在 workspace 下执行构建命令不会读取
.cargo/config.toml 配置文件，见[说明](https://doc.rust-lang.org/cargo/reference/config.html)。
```shell
cd client
# cargo watch -cx run // 运行开发工具并监听代码修改
bacon run
```

### Bacon 
[Bacon](https://dystroy.org/bacon/config/#job-properties) 是一个 Rust 开发工具，详细用法查看[文档](https://dystroy.org/bacon/config/#job-properties)。

