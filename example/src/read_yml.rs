use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};
use serde_json::{from_str as json_from_str, to_string_pretty};
use serde_yaml::from_str as yaml_from_str;
use std::fs::read_to_string;


#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    pub mysql: Mysql,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mysql {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub db: String,
    pub pool_min_idle: u64,
    pub pool_max_open: u64,
    pub timeout_seconds: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Profiles {
    pub active: String,
}

#[derive(Serialize, Deserialize)]
pub struct EnvConfig {
    pub profiles: Profiles,
}

/*
加载环境配置
 */
pub fn load_env_conf() -> Option<EnvConfig> {
    let path = format!("src/resources/application.yml");
    let schema = yaml_from_str::<RootSchema>(&read_to_string(&path).unwrap_or_else(|_| {
        panic!(
            "Error loading configuration file {}, please check the configuration!",
            &path
        )
    }));
    return match schema {
        Ok(json) => {
            let data = to_string_pretty(&json).expect("resources/app.yml file data error！");
            let p: EnvConfig =
                json_from_str(&*data).expect("Failed to transfer JSON data to EnvConfig object！");
            return Some(p);
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    };
}

/*
根据环境配置加载全局配置
action  dev 开始环境 test 测试环境 prod 生产环境
 */
pub fn load_global_config(action: String) -> Option<GlobalConfig> {
    let path = format!("src/resources/application-{}.yml", &action);
    let schema = yaml_from_str::<RootSchema>(&read_to_string(&path).unwrap_or_else(|_| {
        panic!(
            "Error loading configuration file {}, please check the configuration!",
            &path
        )
    }));
    return match schema {
        Ok(json) => {
            let data = to_string_pretty(&json).unwrap_or_else(|_| {
                panic!(
                    "{} file data error！, please check the configuration!",
                    path
                )
            });
            let p = json_from_str(&*data)
                .expect("Failed to transfer JSON data to BriefProConfig object！");
            return Some(p);
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    };
}

/*
先加载环境配置 在根据当前加载的环境 去加载相应的信息
 */
pub fn load_conf() -> Option<GlobalConfig> {
    if let Some(init) = load_env_conf() {
        return load_global_config(init.profiles.active);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_env_conf_mysql() {
        let pro = load_conf();
        println!("{:?}", pro);
        pro.as_ref().map(|a| {
            println!("mysqlConfig:{}", serde_json::to_string(&a.mysql).unwrap());
        });
    }
}
