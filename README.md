 # rust日志库
 - 支持打印日志到标准输出
 - 支持日志打印到文件，支持设置文件重写大小（到达文件设置大小truncate文件）

 # Examples
 ```
 #[macro_use]
 extern crate logger;
 use logger::log;

 log::set_level(log::A_LEVEL);
 // log::set_rotate_size(1024);
 // log::set_filepath("/tmp/rs.dat".to_owned());

 traceln!("{}", "hello world");
 debugln!("{}", "hello world");
 infoln!("{}", "hello world");
 errorln!("{}", "hello world");
 warnln!("{}", "hello world");
 fatalln!("{}", "hello world");
 ```
