extern crate wasm_bindgen;
extern crate web_sys;

mod livro1;
mod livro2;
mod livro3;
mod sons;

use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use livro1::LIVRO_1;
use livro2::LIVRO_2;
use livro3::LIVRO_3;
use sons:: { SOM_INICIO, SOM_TERMINO };

pub struct IntervalHandle {
    interval_id: i32,
    _closure: Closure<dyn FnMut()>,
}

impl Drop for IntervalHandle {
    fn drop(&mut self) {
        let window = web_sys::window().unwrap();
        window.clear_interval_with_handle(self.interval_id);
    }
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let escolher = document.get_element_by_id("escolher").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    let pista = document.get_element_by_id("pista").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    let ver_pista = document.get_element_by_id("verPista").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    escolher.set_hidden(false);
    pista.set_hidden(true);

    let btn_click = Closure::wrap(Box::new(move || {
        usar_pista().expect("Não fez botão");
    }) as Box<dyn FnMut()>);
    ver_pista.set_onclick(Some(btn_click.as_ref().unchecked_ref()));
    btn_click.forget();

    Ok(())
}

pub fn usar_pista() -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let texto_pista = document.get_element_by_id("textoPista").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    let numero_pista = document.get_element_by_id("numeroPista").unwrap().dyn_into::<web_sys::HtmlInputElement>()?;
    let livro1 = document.get_element_by_id("livro1").unwrap().dyn_into::<web_sys::HtmlInputElement>()?;
    let livro2 = document.get_element_by_id("livro2").unwrap().dyn_into::<web_sys::HtmlInputElement>()?;
    let livro3 = document.get_element_by_id("livro3").unwrap().dyn_into::<web_sys::HtmlInputElement>()?;

    let numero_pista_value = numero_pista.value();

    match numero_pista_value.trim().parse::<usize>() {
        Ok(result) => {
            if livro1.checked() {
                if result > 0 && result <= LIVRO_1.len(){
                    texto_pista.set_inner_text(LIVRO_1[result - 1]);
                    carregar_pista()?;
                }
                else {
                    window.alert_with_message(&format!("Escolha um número entre 1 e {}.", LIVRO_1.len()))?;
                }
            }
            if livro2.checked() {
                if result > 0 && result <= LIVRO_2.len(){
                    texto_pista.set_inner_text(LIVRO_2[result - 1]);
                    carregar_pista()?;
                }
                else {
                    window.alert_with_message(&format!("Escolha um número entre 1 e {}.", LIVRO_2.len()))?;
                }
            }
            if livro3.checked() {
                if result > 0 && result <= LIVRO_3.len(){
                    texto_pista.set_inner_text(LIVRO_3[result - 1]);
                    carregar_pista()?;
                }
                else {
                    window.alert_with_message(&format!("Escolha um número entre 1 e {}.", LIVRO_3.len()))?;
                }
            }
        }
        Err(_) => {
            window.alert_with_message("Digite um numero no campo.")?;
        }
    }

    Ok(())
}

pub fn carregar_pista() -> Result<(), JsValue> {
    let som_inicio = HtmlAudioElement::new_with_src(&format!("data:audio/wav;base64,{}", SOM_INICIO));
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let escolher = document.get_element_by_id("escolher").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    let pista = document.get_element_by_id("pista").unwrap().dyn_into::<web_sys::HtmlElement>()?;

    let _ = som_inicio.unwrap().play().unwrap();
    atualizar_tempo_restante(30)?;
    pista.set_hidden(false);
    escolher.set_hidden(true);
    atualizar()?;

    Ok(())
}

pub fn atualizar() -> Result<(), JsValue> {
    let som_termino = HtmlAudioElement::new_with_src(&format!("data:audio/wav;base64,{}", SOM_TERMINO));
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let escolher = document.get_element_by_id("escolher").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    let pista = document.get_element_by_id("pista").unwrap().dyn_into::<web_sys::HtmlElement>()?;

    let tempo = pegar_tempo_restante().unwrap();

    if tempo <= 0 {
        let _ = som_termino.unwrap().play().unwrap();
        pista.set_hidden(true);
        escolher.set_hidden(false);
    } else {
        atualizar_tempo_restante(tempo - 1)?;
        let contagem = Closure::wrap(Box::new(|| {
            atualizar().unwrap();
        }) as Box<dyn FnMut()>);
        window.set_timeout_with_callback_and_timeout_and_arguments_0(contagem.as_ref().unchecked_ref(), 900)?;
        contagem.forget();
    }

    Ok(())
}

pub fn atualizar_tempo_restante (tempo_restante: i8) -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let texto_pista = document.get_element_by_id("tempoRestante").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    
    texto_pista.set_inner_text(&format!("{}", tempo_restante));
    Ok(())
}

pub fn pegar_tempo_restante() -> Result<i8, JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let tempo_restante = document.get_element_by_id("tempoRestante").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    match tempo_restante.inner_text().trim().parse::<i8>() {
        Ok(tempo) => {
            Ok(tempo)
        }
        Err(error) => {
            window.alert_with_message(&format!("{}", error))?;
            Ok(1)
        }
    }
}