extern crate js_sys;
use chrono::prelude::*;
use serde::Serialize;
use yew::prelude::*;

pub struct ButtonOpts<'a> {
    utc_millis: u64,
    file_prefix: &'a str,
    button_id: &'a str,
    a_class: &'a str,
}

pub fn button<T>(data: &T, opts: ButtonOpts) -> Html
where
    T: Serialize,
{
    let dt = Utc.timestamp_millis(opts.utc_millis as i64);
    let formatted_datetime: String = dt.format("%Y%m%d_%H%M%SZ").to_string();
    let filename: String = format!("{}_{}.json", opts.file_prefix, formatted_datetime);
    if let Ok(href) = provide_data(data) {
        html! { <button id=opts.button_id><a href=href download=filename class=opts.a_class>{ "Export 💾" }</a></button> }
    } else {
        html! { <button id=opts.button_id>{ "Unavailable ⛔" }</button>}
    }
}

const FILE_TYPE: &str = "application/json";

fn provide_data<T>(data: &T) -> Result<String, ProvideDataErr>
where
    T: Serialize,
{
    if let Ok(ser) = serde_json::to_string(data) {
        let encoded: String = js_sys::encode_uri_component(&ser).into();

        Ok(format!("data:{};charset=utf-8,{}", FILE_TYPE, encoded))
    } else {
        Err(ProvideDataErr)
    }
}

struct ProvideDataErr;
