### 构建项目

Web 开发环境需要在项目目录中执行命令，在 workspace 下执行构建命令不会读取
.cargo/config.toml 配置文件，见[说明](https://doc.rust-lang.org/cargo/reference/config.html)。
```shell
cd client
# cargo watch -cx run // 运行开发工具并监听代码修改
bacon run
```

### 客户端调试
本项目用rustrover开发并使用其调试工具：
1. 新增项目Cargo运行配置：
    * Command: `run --bin client`
2. 编辑环境变量：
    * 添加 TEST_ID 环境变量，该环境变量会被转换为`u64`，确保该变量能被正确转换

⚠️bevy 开启动态连接(Dynamic Linking)后，调试会报stdlib动态库（大概是这个）没有找到，这时有两个方法：
1. 禁用动态连接
2. 参考这里的[解决方法](https://www.reddit.com/r/bevy/comments/198fu1z/getting_dynamic_linking_debugging_to_work_in/)

### Bacon 
[Bacon](https://dystroy.org/bacon/config/#job-properties) 是一个 Rust 开发工具，详细用法查看[文档](https://dystroy.org/bacon/config/#job-properties)。
bacon.toml 是Bacon的默认配置文件，在有该文件的目录下，命令行中输入 `bacon`启动。
