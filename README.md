# yew-export-button

🚧 _Under construction_ 🚧

This will be a lib which defines the following type of function, but in a generic form. 

```rust
use crate::model::*;
use chrono::prelude::*;
use yew::prelude::*;

pub fn button(inventory: &Inventory, now: UtcMillis) -> Html {
    let dt = Utc.timestamp_millis(now.0 as i64);
    let formatted_datetime: String = dt.format("%Y%m%d_%H%M%SZ").to_string();
    let filename: String = format!("inventory_{}.json", formatted_datetime);
    if let Ok(href) = provide_data(inventory) {
        html! { <button id="exportbutton"><a href=href download=filename class="download">{ "Export 💾" }</a></button> }
    } else {
        html! { <button id="exportbutton">{ "Unavailable ⛔" }</button>}
    }
}

const FILE_TYPE: &str = "application/json";

fn provide_data(state: &Inventory) -> Result<String, ProvideDataErr> {
    if let Ok(ser) = serde_json::to_string(state) {
        let encoded: String = js_sys::encode_uri_component(&ser).into();

        Ok(format!("data:{};charset=utf-8,{}", FILE_TYPE, encoded))
    } else {
        Err(ProvideDataErr)
    }
}

struct ProvideDataErr;
```
