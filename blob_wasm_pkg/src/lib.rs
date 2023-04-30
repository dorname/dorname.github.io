extern  crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsValue;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &JsValue);

    #[wasm_bindgen(js_namespace = Ext)]
    fn create(s: &str,conf:&JsValue);
}
#[derive(Serialize,Deserialize)]
struct Store {
    id:String
}
#[wasm_bindgen]
pub fn test() {
   let obj = Store{
        id:String::from("testStore")
   };
//    let config = JsValue::from_serde(&json!({})).unwrap();
   
   let out_one = JsValue::from_str(&serde_json::to_string(&obj).unwrap());
//    let out_two = JsValue::from_str(&serde_json::to_string(&obj).unwrap());
   log(&out_one);
//    log(&JsValue::from_str("hhhh"));
   create("Ext.Component",&JsValue::from_str("hhhh"));
}
