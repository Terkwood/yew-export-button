extern crate js_sys;
use chrono::prelude::*;
use serde::Serialize;
use yew::prelude::*;

pub fn button<T>(data: &T, now: u64, file_prefix: &str, button_id: &str) -> Html
where
    T: Serialize,
{
    let dt = Utc.timestamp_millis(now as i64);
    let formatted_datetime: String = dt.format("%Y%m%d_%H%M%SZ").to_string();
    let filename: String = format!("{}_{}.json", file_prefix, formatted_datetime);
    if let Ok(href) = provide_data(data) {
        html! { <button id=button_id><a href=href download=filename class="download">{ "Export 💾" }</a></button> }
    } else {
        html! { <button id=button_id>{ "Unavailable ⛔" }</button>}
    }
}

const FILE_TYPE: &str = "application/json";

fn provide_data<T>(state: &T) -> Result<String, ProvideDataErr>
where
    T: Serialize,
{
    if let Ok(ser) = serde_json::to_string(state) {
        let encoded: String = js_sys::encode_uri_component(&ser).into();

        Ok(format!("data:{};charset=utf-8,{}", FILE_TYPE, encoded))
    } else {
        Err(ProvideDataErr)
    }
}

struct ProvideDataErr;
