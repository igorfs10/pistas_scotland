extern crate wasm_bindgen;
extern crate web_sys;

mod livro1;
mod livro2;
mod livro3;

use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use livro1::LIVRO_1;
use livro2::LIVRO_2;
use livro3::LIVRO_3;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let escolher = document.get_element_by_id("escolher").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    let pista = document.get_element_by_id("pista").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    escolher.set_hidden(false);
    pista.set_hidden(true);
    Ok(())
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn usar_pista(numero_livro: u8, numero_pista: usize) -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let escolher = document.get_element_by_id("escolher").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    let pista = document.get_element_by_id("pista").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    // let body = document.body().expect("document should have a body");

    let texto_pista = document.get_element_by_id("textoPista").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    if numero_livro == 1 {
        texto_pista.set_inner_text(LIVRO_1[numero_pista - 1]);
    }
    if numero_livro == 2 {
        texto_pista.set_inner_text(LIVRO_2[numero_pista - 1]);
    }
    if numero_livro == 3 {
        texto_pista.set_inner_text(LIVRO_3[numero_pista - 1]);
    }

    atualizar_tempo_restante(30)?;
    // Thread não funciona com wasm
    // thread::sleep(time::Duration::from_secs(1));

    pista.set_hidden(false);
    escolher.set_hidden(true);

    Ok(())
}

#[wasm_bindgen]
pub fn atualizar_tempo_restante (tempo_restante: i8) -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let texto_pista = document.get_element_by_id("tempoRestante").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    
    texto_pista.set_inner_text(&format!("{}", tempo_restante));
    Ok(())
}