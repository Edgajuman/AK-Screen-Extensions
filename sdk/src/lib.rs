//! Example guest: persistent counter widget and deterministic warm-tone image effect.
wit_bindgen::generate!({ path: "wit", world: "extension" });
use serde_json::{Value, json};
struct Component;
impl Guest for Component {
    fn handle(request:String)->String {
        let Ok(request)=serde_json::from_str::<Value>(&request) else {return json!({"message":"JSON inválido"}).to_string();};
        let mut state=request.get("state").filter(|v|v.is_object()).cloned().unwrap_or_else(||json!({}));
        let count=state["count"].as_u64().unwrap_or(0);
        let count=if request["event"]=="widget" && request["action"]=="increment"{count.saturating_add(1)}else{count};
        state["count"]=json!(count);
        json!({"state":state,"widgets":[{"id":"counter","title":"WASM Counter","controls":[
            {"id":"count","kind":"label","label":"Contador persistente","value":count.to_string()},
            {"id":"increment","kind":"button","label":"+1","action":"increment"}
        ]}]}).to_string()
    }
    fn render(width:u32,height:u32,mut pixels:Vec<f32>,params:String,_time:i64)->Result<Vec<f32>,String> {
        if width.checked_mul(height).and_then(|p|p.checked_mul(4)).map(|n|n as usize)!=Some(pixels.len()) {return Err("Dimensiones inválidas".into());}
        let config:Value=serde_json::from_str(&params).map_err(|_|"Parámetros JSON inválidos")?;
        let warmth=config["warmth"].as_f64().unwrap_or(0.08).clamp(-0.5,0.5) as f32;
        for p in pixels.chunks_exact_mut(4) {p[0]=(p[0]+warmth*p[3]).max(0.0);p[2]=(p[2]-warmth*p[3]).max(0.0);}
        Ok(pixels)
    }
}
export!(Component);
