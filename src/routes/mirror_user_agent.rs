use axum::http::HeaderMap;

pub async fn mirror_user_agent(headers: HeaderMap) -> String {
    let user_agent_value = headers.get("user-agent").unwrap();
    user_agent_value.to_str().unwrap().to_string()
}
