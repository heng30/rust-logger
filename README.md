 # rust日志库
 - 支持打印日志输出到标准或文件
 - 支持设置文件重写大小
 - 支持日志级别
 - 线程安全

 ## Examples：
 ```
 #[macro_use]
 extern crate logger;
 use logger::log;

 log::set_level(log::A_LEVEL);
 log::set_size(1024);
 log::set_filepath("/tmp/rs.dat".to_owned());

 traceln!("{}", "hello world");
 debugln!("{}", "hello world");
 infoln!("{}", "hello world");
 errorln!("{}", "hello world");
 warnln!("{}", "hello world");
 fatalln!("{}", "hello world");
 ```

## 输出：
 ``` text
[T] [2020-04-16-23:16:08] [bin/main.rs/{{closure}}:22]: hello world
[D] [2020-04-16-23:16:08] [bin/main.rs/{{closure}}:23]: hello world
[I] [2020-04-16-23:16:08] [bin/main.rs/{{closure}}:24]: hello world
[W] [2020-04-16-23:16:08] [bin/main.rs/{{closure}}:25]: hello world
[E] [2020-04-16-23:16:08] [bin/main.rs/{{closure}}:26]: hello world
[F] [2020-04-16-23:16:08] [bin/main.rs/{{closure}}:27]: hello world
 ```
