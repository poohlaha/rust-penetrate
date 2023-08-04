# rust-penetrate
  使用 Rust 来穿透。

### conf.ini
在根目录创建 `conf.ini` 文件, 并添加如下配置(示例):
   ```ini
   [server]
   host = "127.0.0.1"
   port = 7878

   [interceptor]
   paths = /v1/*,/api/*
   ```

其中：
- server
    - host: 服务器地址, 默认 127.0.0.1
    - port: 服务器启动端口号, 默认 7878
- interceptor
    - paths: 路径, 以 `,` 分割

