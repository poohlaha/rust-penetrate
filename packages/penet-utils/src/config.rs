//! 读取目录下的 conf.ini 文件, 使用 lazy_static 共享文件属性

use crate::logger::Logger;
use crate::utils::Utils;
use crate::LOGGER_PREFIX;
use ini::{Ini, ParseError, Properties};
use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Config {
    pub instance: Option<ConfigIni>,
}

// 服务器配置
#[derive(Clone, Debug, Default)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

// 拦截器设置
#[derive(Clone, Debug, Default)]
pub struct Interceptor {
    pub paths: Vec<String>
}
// conf.ini 配置
#[derive(Clone, Debug)]
pub struct ConfigIni {
    pub server: Server,
    pub interceptor: Interceptor,
}

lazy_static! {
    static ref GLOBAL_CONFIG: Mutex<Option<ConfigIni>> = Mutex::new(None);
}

impl Config {
    pub fn new() -> Self {
        let mut config = Config { instance: None };

        let mut config_ = GLOBAL_CONFIG.lock().unwrap();
        if let Some(config_) = &*config_ {
            config.instance = Some(config_.clone());
        } else {
            let new_config = get_config();
            config.instance = new_config.clone();
            *config_ = new_config.clone();
        }

        return config;
    }
}

/// 获取 config 配置
fn get_config() -> Option<ConfigIni> {
    let current_dir = env::current_dir().unwrap();
    Logger::debug(LOGGER_PREFIX, file!(), line!(), Some("current_dir"), current_dir.clone().as_path().to_str().unwrap());
    let mut path = PathBuf::from(current_dir);
    path.push("conf.ini");

    // 判断路径是否存在
    if !path.exists() {
        Logger::error(LOGGER_PREFIX, file!(), line!(), "current dir has no `conf.ini`, please check !");
        return None;
    }

    let file_path = path.as_path().to_str().unwrap();
    let contents = Utils::read_file(file_path);
    if contents.is_empty() {
        return None;
    }

    let conf = match Ini::load_from_str(&contents) {
        Ok(conf) => Some(conf),
        Err(err) => {
            Logger::debug_object::<ParseError>(LOGGER_PREFIX, file!(), line!(), "analysis `conf.ini` error", err);
            None
        }
    };

    if conf.is_none() {
        return None;
    }

    let conf = conf.unwrap();

    // server
    let mut server = Server::default();
    let server_section = get_section(&conf, "server");
    if !server_section.is_none() {
        let server_section = server_section.unwrap();
        let host = server_section.get("host").unwrap_or("").to_string();
        let port = server_section.get("port").unwrap_or("").to_string();
        if !host.is_empty() {
            server.host = host;
        }

        if !port.is_empty() {
            server.port = port.parse::<u16>().unwrap();
        }
    }

    // interceptor
    let mut interceptor = Interceptor::default();
    let interceptor_section = get_section(&conf, "interceptor");
    if !interceptor_section.is_none() {
        let interceptor_section = interceptor_section.unwrap();
        if let Some(path_str) = interceptor_section.get("paths") {
            let paths: Vec<String> = path_str.split(',').map(|p| p.trim().to_string()).collect();
            interceptor.paths = paths
        }
    }

    return Some(ConfigIni { server, interceptor });
}

/// 获取节点
fn get_section(conf: &Ini, section_name: &str) -> Option<Properties> {
    return match conf.section(Some(section_name)) {
        None => {
            let error_msg = format!("analysis `conf.ini` error, can not get `{}` section", section_name);
            Logger::error(LOGGER_PREFIX, file!(), line!(), &error_msg);
            None
        }
        Some(server) => Some(server.clone()),
    };
}