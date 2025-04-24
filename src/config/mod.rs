pub mod database_config;

pub fn get_var_or_default(key: &str, def_value: &str) -> String {
    let value = dotenvy::var(key);
    if let Ok(unwrapped_value) = value {
        return unwrapped_value;
    }
    def_value.to_string()
}

pub fn get_var(key: &str) -> String {
    let value = dotenvy::var(key);
    if let Ok(unwrapped_value) = value {
        unwrapped_value
    } else {
        panic!("Error in getting value from .env by key[{}]\n[{}]", key, value.unwrap_err());
    }
}
