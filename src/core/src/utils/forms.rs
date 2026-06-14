use leptos::{html::Form, prelude::*};
use serde::{Deserialize, de::DeserializeOwned};
use web_sys::{CustomEvent, CustomEventInit, Event, EventInit, EventTarget};

use js_sys::{Array, IntoIter, wasm_bindgen::JsValue};
use serde_json::{Map, Number, Value};
use web_sys::FormData;

#[derive(Debug)]
pub enum FormDeserializeError {
    NoFormData,
    IterationFailed,
    EntryParseFailed,
    DeserializeFailed(serde_json::Error, Value),
}

impl std::fmt::Display for FormDeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFormData => write!(f, "Could not get FormData from form ref"),
            Self::IterationFailed => write!(f, "Could not iterate FormData entries"),
            Self::EntryParseFailed => write!(f, "Failed to parse a FormData entry"),
            Self::DeserializeFailed(err, val) => {
                write!(
                    f,
                    "Deserialization failed: {err}\nIntermediate JSON: {val:#}"
                )
            }
        }
    }
}

pub fn get_form_data_from_form_ref(form_ref: &NodeRef<Form>) -> Option<FormData> {
    let form = form_ref.to_owned().get_untracked()?;
    let form_data = FormData::new_with_form(&form).ok()?;
    Some(form_data)
}

/// `deserialize_bool` - Whether to deserialize boolean values
///
/// `vec_fields` - The fields that should be deserialized as Vec<String>, e.g. Checkbox fields
pub fn deserialize_form<T: DeserializeOwned>(
    form_ref: &NodeRef<Form>,
    deserialize_bool: bool,
    vec_fields: Option<&[&str]>,
) -> Option<T> {
    deserialize_form_checked(form_ref, deserialize_bool, vec_fields).ok()
}

/// This is a counterpart to the `deserialize_form` utility (which returns `Option<T>`).
///
/// This abstraction enables easy debugging by providing errors on why the deserialization failed.
///
/// `deserialize_bool` - Whether to deserialize boolean values
///
/// `vec_fields` - The fields that should be deserialized as Vec<String>, e.g. Checkbox fields
pub fn deserialize_form_checked<T: DeserializeOwned>(
    form_ref: &NodeRef<Form>,
    deserialize_bool: bool,
    vec_fields: Option<&[&str]>,
) -> Result<T, FormDeserializeError> {
    let form_data =
        get_form_data_from_form_ref(form_ref).ok_or(FormDeserializeError::NoFormData)?;

    let mut entries = js_sys::try_iter(&form_data)
        .ok()
        .flatten()
        .ok_or(FormDeserializeError::IterationFailed)?;

    let mut map = Map::new();

    parse_form(&mut map, &mut entries, deserialize_bool, vec_fields)
}

/// A reusable function to parse the FormData
fn parse_form<T: DeserializeOwned>(
    map: &mut Map<String, Value>,
    entries: &mut IntoIter,
    deserialize_bool: bool,
    vec_fields: Option<&[&str]>,
) -> Result<T, FormDeserializeError> {
    for entry in entries {
        let pair = entry.ok().ok_or(FormDeserializeError::EntryParseFailed)?;
        let arr = Array::from(&pair);
        let key = arr
            .get(0)
            .as_string()
            .ok_or(FormDeserializeError::EntryParseFailed)?;
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

    let value = Value::Object(map.to_owned());
    serde_json::from_value::<T>(value.clone())
        .map_err(|e| FormDeserializeError::DeserializeFailed(e, value))
}

/// `deserialize_bool` - Whether to deserialize boolean values
///
/// `vec_fields` - The fields that should be deserialized as Vec<String>, e.g. Checkbox fields
///
/// This is suitable when you want to append to FormData and then deserialize the final form value
pub fn deserialize_form_data<T: DeserializeOwned>(
    form_data: &FormData,
    deserialize_bool: bool,
    vec_fields: Option<&[&str]>,
) -> Option<T> {
    deserialize_form_data_checked(form_data, deserialize_bool, vec_fields).ok()
}

/// This is a counterpart to the `deserialize_form_data` utility (which returns `Option<T>`).
///
/// This abstraction enables easy debugging by providing errors on why the deserialization failed.
///
/// `deserialize_bool` - Whether to deserialize boolean values
///
/// `vec_fields` - The fields that should be deserialized as Vec<String>, e.g. Checkbox fields
pub fn deserialize_form_data_checked<T: DeserializeOwned>(
    form_data: &FormData,
    deserialize_bool: bool,
    vec_fields: Option<&[&str]>,
) -> Result<T, FormDeserializeError> {
    let mut entries = js_sys::try_iter(&form_data)
        .ok()
        .flatten()
        .ok_or(FormDeserializeError::IterationFailed)?;

    let mut map = Map::new();

    parse_form(&mut map, &mut entries, deserialize_bool, vec_fields)
}

/// Fire DOM events which take place on an `EventTarget`s
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

/// Fire custom DOM events and attach custom data to an events generated by an application.
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
