use teloxide::types::Message;

pub fn get_param(msg: &Message, error_msg: &'static str) -> anyhow::Result<String> {
    let params: Vec<String> = msg
        .text()
        .as_ref()
        .map(|text| text.split_whitespace().skip(1).map(String::from).collect())
        .unwrap_or_default();

    if params.is_empty() {
        return Err(anyhow::anyhow!(error_msg));
    }

    Ok(params.join(" "))
}

pub fn get_n_params(
    msg: &Message,
    n: usize,
    error_msg: &'static str,
) -> anyhow::Result<Vec<String>> {
    let params: Vec<String> = msg
        .text()
        .as_ref()
        .map(|text| text.split_whitespace().skip(1).map(String::from).collect())
        .unwrap_or_default();

    if params.len() < n {
        return Err(anyhow::anyhow!(error_msg));
    }

    Ok(params)
}
