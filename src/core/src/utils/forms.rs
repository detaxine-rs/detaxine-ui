use leptos::{html::Form, prelude::*};
use serde::de::DeserializeOwned;
use web_sys::{CustomEvent, CustomEventInit, Event, EventInit, EventTarget};

use js_sys::Array;
use serde_json::{Map, Number, Value};
use web_sys::FormData;

pub fn get_form_data_from_form_ref(form_ref: &NodeRef<Form>) -> Option<FormData> {
    let form = form_ref.to_owned().get_untracked()?;
    let form_data = FormData::new_with_form(&form).ok()?;
    Some(form_data)
}

/// `deserialize_bool` - Whether to deserialize boolean values
///
/// `vec_fields` - The fields that should be deserialized as Vec<String>, e.g. Checkbox fields
pub fn deserialize_form_data_to_struct<T: DeserializeOwned>(
    form_data: &FormData,
    deserialize_bool: bool,
    vec_fields: Option<&[&str]>,
) -> Option<T> {
    if let Some(entries) = js_sys::try_iter(form_data).ok()? {
        let mut map = Map::new();

        for entry in entries {
            let pair = entry.ok()?;
            let arr = Array::from(&pair);
            let key = arr.get(0).as_string()?;
            let value = arr.get(1);

            let is_vec_field = vec_fields
                .map(|fields| fields.contains(&key.as_str()))
                .unwrap_or(false);

            if let Some(s) = value.as_string() {
                // Convert to bool, null, or string
                let val = if s.is_empty() {
                    Value::Null
                } else if deserialize_bool && (s == "true" || s == "false") {
                    Value::Bool(s.parse::<bool>().unwrap_or_default())
                } else {
                    let trimmed = s.trim();
                    // Attempt integer parsing first
                    if let Ok(i) = trimmed.parse::<i64>() {
                        Value::Number(Number::from(i))
                    }
                    // Attempt float parsing
                    else if let Ok(f) = trimmed.parse::<f64>() {
                        Number::from_f64(f)
                            .map(Value::Number)
                            .unwrap_or(Value::String(trimmed.to_string()))
                    } else {
                        Value::String(trimmed.to_string())
                    }
                };

                // Merge into existing entry if present
                match map.get_mut(&key) {
                    Some(existing) => match existing {
                        Value::Array(arr) => {
                            arr.push(val);
                        }
                        prev => {
                            // Convert single previous value into array
                            let new_arr = vec![prev.clone(), val];
                            *prev = Value::Array(new_arr);
                        }
                    },
                    None => {
                        if is_vec_field {
                            // Always store as array for defined checkbox fields
                            map.insert(key, Value::Array(vec![val]));
                        } else {
                            map.insert(key, val);
                        }
                    }
                }
            }
        }

        let value = Value::Object(map);
        serde_json::from_value(value).ok()
    } else {
        None
    }
}

pub fn fire_bubbled_and_cancelable_event<T>(
    event_type: &str,
    bubbles: bool,
    cancelable: bool,
    element: &T,
) -> ()
where
    T: AsRef<EventTarget>,
{
    let init = EventInit::new();
    init.set_bubbles(bubbles);
    init.set_cancelable(cancelable);

    let _event = match Event::new_with_event_init_dict(event_type, &init) {
        Ok(ev) => {
            element.as_ref().dispatch_event(&ev).unwrap_or_default();
        }
        Err(_e) => {}
    };
}

pub fn fire_custom_bubbled_and_cancelable_event<T>(
    event_type: &str,
    bubbles: bool,
    cancelable: bool,
    element: &T,
) -> ()
where
    T: AsRef<EventTarget>,
{
    let init = CustomEventInit::new();
    init.set_bubbles(bubbles);
    init.set_cancelable(cancelable);

    let _event = match CustomEvent::new_with_event_init_dict(event_type, &init) {
        Ok(ev) => {
            element.as_ref().dispatch_event(&ev).unwrap_or_default();
        }
        Err(_e) => {}
    };
}
