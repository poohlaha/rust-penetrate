//! 公共类
use crate::LOGGER_PREFIX;
use colored::*;
use std::{fs, io};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read, Write};
use std::os::unix::prelude::PermissionsExt;
use std::path::Path;
use std::process::{Command, Output};

pub struct Utils;

impl Utils {
    /// get file
    fn get_file(file_path: &str) -> Option<File> {
        println!("{} read file, path is: {}", LOGGER_PREFIX.cyan().bold(), file_path.to_string().magenta().bold());
        let file = File::open(&file_path);
        let file = match file {
            Ok(f) => Some(f),
            Err(err) => {
                println!("{} open {} error: {:?}", LOGGER_PREFIX.cyan().bold(), file_path.to_string().magenta().bold(), err);
                None
            }
        };

        return file;
    }

    /// 读取文件
    pub fn read_file(file_path: &str) -> String {
        let file = Self::get_file(file_path);

        if file.is_none() {
            return String::new();
        }

        let mut contents = String::new();
        let err_msg = format!("{} read {} error !", LOGGER_PREFIX.cyan().bold(), file_path.to_string().magenta().bold());
        file.unwrap().read_to_string(&mut contents).expect(&err_msg);

        if contents.is_empty() {
            return String::new();
        }

        return contents;
    }

    /// 读取文件行数
    pub fn read_file_lines(file_path: &str) -> Option<Lines<BufReader<File>>> {
        let file = Self::get_file(file_path);
        if file.is_none() {
            return None;
        }

        let reader = BufReader::new(file.unwrap());
        return Some(reader.lines());
    }

    /// 设置文件的权限
    #[allow(dead_code)]
    pub fn set_file_permission(file_path: &str, permission_code: Option<u32>) -> bool {
        let path = Path::new(file_path);
        if !path.exists() {
            println!("{} Failed to set file permission: {}", LOGGER_PREFIX.cyan().bold(), file_path.to_string().magenta().bold());
            return false;
        }

        let code = if permission_code.is_none() { 0o666 } else { permission_code.unwrap() };
        if let Ok(metadata) = fs::metadata(&file_path) {
            let mut permissions = metadata.permissions();
            permissions.set_readonly(false);
            permissions.set_mode(code);
            match fs::set_permissions(&file_path, permissions) {
                Ok(_) => return true,
                Err(err) => {
                    println!("{} Failed to set file permission: {}", LOGGER_PREFIX.cyan().bold(), err);
                    return false;
                }
            }
        }

        println!("{} Failed to set file permission: {}", LOGGER_PREFIX.cyan().bold(), file_path.to_string().magenta().bold());
        return false;
    }

    /// 写入到文件
    pub fn write_file(str: &str, file_path: &str) -> bool {
        if str.is_empty() || file_path.is_empty() {
            return false;
        }

        let path = Path::new(file_path);
        if !path.exists() {
            println!("{} Write to file error, file path: {} not exists !", LOGGER_PREFIX.cyan().bold(), file_path.to_string().magenta().bold());
            return false;
        }

        let file = match fs::OpenOptions::new().write(true).truncate(true).open(file_path) {
            Ok(file) => Some(file),
            Err(err) => {
                println!("{} open {} error: {:?}", LOGGER_PREFIX.cyan().bold(), file_path.to_string().magenta().bold(), err);
                None
            }
        };

        if file.is_none() {
            return false;
        }

        let mut file = file.unwrap();
        match file.write(str.as_bytes()) {
            Ok(_) => true,
            Err(err) => {
                println!("{} write to file path: {} error: {:?}", LOGGER_PREFIX.cyan().bold(), file_path.to_string().magenta().bold(), err);
                return false;
            }
        }
    }

    /// 执行命令
    pub fn exec_command(command: &str) -> bool {
        if command.is_empty() {
            println!("{} command is empty !", LOGGER_PREFIX.cyan().bold());
            return false;
        }

        let output: Result<Output, io::Error>;
        let _command = command.clone().replace("\n", " && ");

        // windows 通过 cmd /C 执行多条命令: cd c:\\usr\\local\\nginx\\sbin/ && nginx
        #[cfg(target_os = "windows")]
        {
            output = Command::new("cmd").args(&["/C", &_command]).output();
        }

        // linux|macos 通过 shell -c 执行多条命令: cd /usr/local/nginx/sbin/\n./nginx
        #[cfg(target_os = "macos")]
        {
            output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
        }

        #[cfg(target_os = "linux")]
        {
            output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
        }

        let output = match output {
            Ok(output) => Some(output),
            Err(err) => {
                println!("{} get port error: {:?}", LOGGER_PREFIX.cyan().bold(), err);
                None
            }
        };

        let mut flag = false;
        if let Some(output) = output {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout).to_string();
                println!("{} exec command success: {:?}", LOGGER_PREFIX.cyan().bold(), output_str);
                flag = true;
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("{} exec command error: {:?}", LOGGER_PREFIX.cyan().bold(), stderr);
                flag = false;
            }
        }

        return flag;
    }
}
