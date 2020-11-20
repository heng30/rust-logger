//! # rust日志库
//! - 支持打印日志到标准输出
//! - 支持日志打印到文件，支持设置文件重写大小（到达文件设置大小truncate文件）
//!
//! # Examples
//!
//! #[macro_use]
//!
//! extern crate logger;
//!
//! use logger::log;
//!
//! traceln!("{}", "hello world");
//!
//! debugln!("{}", "hello world");
//!
//! infoln!("{}", "hello world");
//!
//! errorln!("{}", "hello world");
//!
//! warnln!("{}", "hello world");
//!
//! fatalln!("{}", "hello world");
//!
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

/// 日志级别: trace
pub const T_LEVEL: u8 = 1u8;
/// 日志级别:  debug
pub const D_LEVEL: u8 = 2u8;
/// 日志级别: info
pub const I_LEVEL: u8 = 4u8;
/// 日志级别: warning
pub const W_LEVEL: u8 = 8u8;
/// 日志级别: error
pub const E_LEVEL: u8 = 16u8;
/// 日志级别: fatal
pub const F_LEVEL: u8 = 32u8;
/// 日志级别: none
pub const N_LEVEL: u8 = 0u8;
/// 日志级别: all
pub const A_LEVEL: u8 = 63u8;

/// 日志信息
///
/// filepath: 文件路径，不设置则为打印到标准输出
/// level: 日志级别，采用或方式设置 'T_LEVEL | D_LEVEL' 表示设置trace和debug级别
/// size: 日志文件重写大小，单位：byte
#[derive(Debug)]
pub struct Logger {
    pub filepath: String,
    level: u8,
    size: u64,
}

lazy_static! {
    /// 全局变量，单例模式，线程安全
    pub static ref LOGGER: Arc<Mutex<Logger>> = Arc::new(Mutex::new(Logger {
        filepath: "".to_string(),
        level: A_LEVEL,
        size: 10 * 1024 * 1024,
    }));
}

/// 获取调用程序名
#[doc(hidden)]
#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let name = &name[..name.len() - 3];
        match name.rfind("::") {
            Some(index) => &name[index + 2..name.len()],
            _ => &name[..name.len()],
        }
    }};
}

/// 设置日志文件路径
pub fn set_filepath(filepath: String) {
    LOGGER.lock().expect("logger get lock failed").filepath = filepath;
}

/// 设置日志级别
pub fn set_level(level: u8) {
    LOGGER.lock().expect("logger get lock failed").level = level;
}

/// 设置日志文件重写大小
pub fn set_size(size: u64) {
    LOGGER.lock().expect("logger get lock failed").size = size;
}

/// 打印filepath，level，size 信息
pub fn dump() {
    let lger = &LOGGER.lock().expect("logger get lock failed");
    println!("{:#?}", lger);
}

// 是否打印日志; **该接口不应被用户调用**
#[doc(hidden)]
pub fn can_log(lger: &Logger, level: u8) -> bool {
    lger.level & level != 0
}

// 是否打印到标准输出; 该接口不应被用户调用
#[doc(hidden)]
pub fn is_stdout(lger: &Logger) -> bool {
    lger.filepath.len() == 0
}

// 获取日志级别对应的字符; 该接口不应被用户调用
#[doc(hidden)]
pub fn level_flag(level: u8) -> char {
    match level {
        T_LEVEL => 'T',
        D_LEVEL => 'D',
        I_LEVEL => 'I',
        W_LEVEL => 'W',
        E_LEVEL => 'E',
        F_LEVEL => 'F',
        _ => 'U',
    }
}

// 检查记录日志的目录和文件是否存在，权限，大小等，并创建文件， 为记录日志做准备; 该接口不应被用户调用
#[doc(hidden)]
pub fn before_log(lger: &Logger) -> io::Result<()> {
    let filepath = &lger.filepath;
    let path = Path::new(filepath);
    let dir = path.parent();
    match dir {
        Some(d) => {
            if !d.is_dir() {
                fs::create_dir_all(d)?;
            }
        }

        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("filepath: {} is invalid", filepath),
            ));
        }
    }

    if !path.is_file() {
        fs::File::create(filepath)?;
    } else {
        if path.metadata()?.len() > lger.size {
            OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(filepath)?;
        }
    }

     Ok(())
}

// 打印日志，内部调用; 该接口不应被用户调用
#[doc(hidden)]
#[macro_export]
macro_rules! log {
    ($lflag: expr, $($arg:tt)*) => {{
        use $crate::chrono::{format, offset::Local};
        use $crate::log;
        use std::fs::OpenOptions;
        use std::io::Write;

        let prefix = format!(
            "[{}] [{}] [{}/{}:{}]: ",
            log::level_flag($lflag),
            Local::now().format("%F-%X"),
            file!(),
            function_name!(),
            line!()
        );

        let lger = log::LOGGER.lock().expect("logger get lock failed");

        if log::can_log(&lger, $lflag) {
            if log::is_stdout(&lger) {
                print!("{}", prefix);
                println!($($arg)*);
                std::io::stdout().flush().unwrap_or_else(|e| {
                    println!("stdout flush error: {:?}", e);
                });
            } else {
                let filepath = &lger.filepath;
                match log::before_log(&lger) {
                    Ok(_)  => {
                        let mut file = OpenOptions::new().write(true).append(true).open(filepath);

                        match file {
                            Ok(mut f) => {
                                write!(f, "{}", prefix)
                                    .unwrap_or_else(|e| println!("write error: {:?}", e));
                                writeln!(f, $($arg)*)
                                    .unwrap_or_else(|e| println!("writeln error: {:?}", e));
                                f.flush().unwrap_or_else(|e| {
                                    println!("file flush error: {:?}", e);
                                });
                            }
                            Err(e) => {
                                println!("open {} error: {:?}", filepath, e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("before_log set filepath: {}, error: {:?}", filepath, e);
                    }
                }
            }
        }
    };};
}

/// 打印日志
#[macro_export]
macro_rules! traceln {
    ($($arg:tt)*) => {{
        log!($crate::log::T_LEVEL, $($arg)*);
    }};
}

/// 打印日志
#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => {{
        log!($crate::log::D_LEVEL, $($arg)*);
    }};
}

/// 打印日志
#[macro_export]
macro_rules! infoln {
    ($($arg:tt)*) => {{
        log!($crate::log::I_LEVEL, $($arg)*);
    }};
}

/// 打印日志
#[macro_export]
macro_rules! warnln {
    ($($arg:tt)*) => {{
        log!($crate::log::W_LEVEL, $($arg)*);
    }};
}

/// 打印日志
#[macro_export]
macro_rules! errorln {
    ($($arg:tt)*) => {{
        log!($crate::log::E_LEVEL, $($arg)*);
    }};
}

/// 打印日志
#[macro_export]
macro_rules! fatalln {
    ($($arg:tt)*) => {{
        log!($crate::log::F_LEVEL, $($arg)*);
    }};
}
