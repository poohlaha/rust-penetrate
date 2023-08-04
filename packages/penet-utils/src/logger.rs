//! 日志

use colored::*;
use std::fmt::Debug;
pub struct Logger;

impl Logger {
    /// 打印错误日志
    pub fn error(prefix: &str, file: &str, line: u32, error_msg: &str) {
        if error_msg.is_empty() {
            return;
        }

        println!(
            "{} [{} {}, {} {}] {}",
            String::from(prefix).cyan().bold(),
            String::from("File:").cyan().bold(),
            file.green().bold(),
            String::from("Line:").cyan().bold(),
            line.to_string().green().bold(),
            String::from(error_msg).red().bold()
        );
    }

    /// 打印错误日志 -- 对象
    pub fn error_object<T>(prefix: &str, file: &str, line: u32, name: &str, value: T)
    where
        T: Debug,
    {
        if name.is_empty() {
            return;
        }

        println!(
            "{} [{} {}, {} {}] {}: {:#?}",
            String::from(prefix).cyan().bold(),
            String::from("File:").cyan().bold(),
            file.green().bold(),
            String::from("Line:").cyan().bold(),
            line.to_string().green().bold(),
            String::from(name).red().bold(),
            value
        );
    }

    /// 打印调试日志
    pub fn debug(prefix: &str, file: &str, line: u32, name: Option<&str>, value: &str) {
        if value.is_empty() {
            return;
        }

        let mut msg: String = String::new();
        if name.is_none() {
            msg = format!(
                "{} [{} {}, {} {}] {}",
                String::from(prefix).cyan().bold(),
                String::from("File:").cyan().bold(),
                file.green().bold(),
                String::from("Line:").cyan().bold(),
                line.to_string().green().bold(),
                String::from(value).magenta().bold()
            );
        } else {
            let name = name.unwrap_or("");
            msg = format!(
                "{} [{} {}, {} {}] {}: {}",
                String::from(prefix).cyan().bold(),
                String::from("File:").cyan().bold(),
                file.green().bold(),
                String::from("Line:").cyan().bold(),
                line.to_string().green().bold(),
                name,
                String::from(value).magenta().bold()
            );
        }

        println!("{}", msg);
    }

    /// 打印 object 对象
    pub fn debug_object<T>(prefix: &str, file: &str, line: u32, name: &str, value: T)
    where
        T: Debug,
    {
        println!(
            "{} [{} {}, {} {}] {}: {:#?}",
            String::from(prefix).cyan().bold(),
            String::from("File:").cyan().bold(),
            file.green().bold(),
            String::from("Line:").cyan().bold(),
            line.to_string().green().bold(),
            name,
            value
        );
    }
}
