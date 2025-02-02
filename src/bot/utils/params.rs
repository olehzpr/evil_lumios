use std::str::FromStr;

use anyhow::anyhow;
use teloxide::types::Message;

pub fn get_param<T>(msg: &Message) -> anyhow::Result<T>
where
    T: FromStr,
{
    let params: Vec<String> = msg
        .text()
        .as_ref()
        .map(|text| text.split_whitespace().skip(1).map(String::from).collect())
        .unwrap_or_default();

    if params.is_empty() {
        return Err(anyhow!("parameter is empty"));
    }
    let param_value = params.join(" ");
    let parsed_param = param_value.parse::<T>();

    match parsed_param {
        Ok(value) => Ok(value),
        Err(_) => Err(anyhow!("parameter couldn't be parsed")),
    }
}

pub fn get_n_params<T>(msg: &Message, n: usize) -> anyhow::Result<Vec<T>>
where
    T: FromStr,
{
    let params: Vec<String> = msg
        .text()
        .as_ref()
        .map(|text| text.split_whitespace().skip(1).map(String::from).collect())
        .unwrap_or_default();

    if params.len() < n {
        return Err(anyhow!("not enough parameters"));
    }

    let mut parsed_params = Vec::new();
    for param in params.into_iter().take(n) {
        match param.parse::<T>() {
            Ok(value) => parsed_params.push(value),
            Err(_) => return Err(anyhow!("parameter couldn't be parsed")),
        }
    }

    Ok(parsed_params)
}
