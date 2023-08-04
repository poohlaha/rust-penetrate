//! 拦截器, 过滤路径等

use crate::LOGGER_PREFIX;
use colored::*;

pub struct Interceptor {
    pub(crate) list: Vec<String>
}

impl Interceptor {
    pub fn new() -> Self {
        Self {
            list: Vec::new()
        }
    }

    /// 获取拦截路径, 去除重复
    pub fn get_path(&mut self, paths: &Vec<String>) {
        let mut list: Vec<String> = Vec::new();

        for path in paths.iter() {
            let mut path = path.trim().to_string();
            if path.ends_with("/") {
                path = path.trim_end_matches('/').to_string();
            }

            let has_in_path = self.find_path(&list, &path);
            if !has_in_path {
                list.push(path.to_string());
            }
        }

        println!("{} interceptor paths: {:#?}", LOGGER_PREFIX.cyan().bold(), list);
        self.list = list;
    }

    /// 查找路径是否需要拦截
    pub fn find_path(&self, list: &Vec<String>, path: &str) -> bool {
        if list.is_empty() {
            println!("{} interceptor has no paths, skip !", LOGGER_PREFIX.cyan().bold());
            return false
        }

        let values = list.iter().find(|p| {
            if p.ends_with("*") {
                if p.starts_with(path) {
                    return true;
                }
            } else if p.to_string() == path.to_string() {
                return true;
            }

            return false
        });

        if let Some(value) = values {
            return value.is_empty()
        }

        return false
    }
}