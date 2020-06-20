extern crate wasm_bindgen;
extern crate web_sys;

mod livros;
mod sons;

use wasm_bindgen::prelude::{ Closure, wasm_bindgen, JsValue };
use web_sys::{ window, HtmlAudioElement };
use wasm_bindgen::JsCast;
use livros::*;
use sons:: { SOM_INICIO, SOM_TERMINO };

// Importa a função main para ser executada quando a página abre
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let escolher = document.get_element_by_id("escolher").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    let pista = document.get_element_by_id("pista").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    let ver_pista = document.get_element_by_id("verPista").unwrap().dyn_into::<web_sys::HtmlElement>()?;

    pista.set_hidden(true);
    escolher.set_hidden(false);

    // Callback para o clique do botão
    let btn_click = Closure::wrap(Box::new(move || {
        usar_pista().expect("Não fez botão");
    }) as Box<dyn FnMut()>);

    ver_pista.set_onclick(Some(btn_click.as_ref().unchecked_ref()));
    btn_click.forget();

    Ok(())
}

// Função executada quando clica no botão de ver pista
fn usar_pista() -> Result<(), JsValue> {
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
                    window.alert_with_message(&format!("Digite um número entre 1 e {}.", LIVRO_1.len()))?;
                }
            }
            if livro2.checked() {
                if result > 0 && result <= LIVRO_2.len(){
                    texto_pista.set_inner_text(LIVRO_2[result - 1]);
                    carregar_pista()?;
                }
                else {
                    window.alert_with_message(&format!("Digite um número entre 1 e {}.", LIVRO_2.len()))?;
                }
            }
            if livro3.checked() {
                if result > 0 && result <= LIVRO_3.len(){
                    texto_pista.set_inner_text(LIVRO_3[result - 1]);
                    carregar_pista()?;
                }
                else {
                    window.alert_with_message(&format!("Digite um número entre 1 e {}.", LIVRO_3.len()))?;
                }
            }
        }
        Err(_) => {
            if livro1.checked(){
                window.alert_with_message(&format!("Digite um número entre 1 e {}.", LIVRO_1.len()))?;
            }
            if livro2.checked(){
                window.alert_with_message(&format!("Digite um número entre 1 e {}.", LIVRO_2.len()))?;
            }
            if livro3.checked(){
                window.alert_with_message(&format!("Digite um número entre 1 e {}.", LIVRO_3.len()))?;
            }
        }
    }

    Ok(())
}

// Função que carrega a pista na tela
fn carregar_pista() -> Result<(), JsValue> {
    let som_inicio = HtmlAudioElement::new_with_src(&format!("data:audio/wav;base64,{}", SOM_INICIO))?;

    let comeca_pista = Closure::wrap(Box::new(|| {
        let _ = mudar_para_pista();
    }) as Box<dyn FnMut()>);

    let _ = som_inicio.play().unwrap().finally(&comeca_pista);
    comeca_pista.forget();

    Ok(())
}

// Muda e seta a tela que mostra a pista  para usar como callback
fn mudar_para_pista() -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let escolher = document.get_element_by_id("escolher").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    let pista = document.get_element_by_id("pista").unwrap().dyn_into::<web_sys::HtmlElement>()?;

    atualizar_tempo_restante(30)?;
    escolher.set_hidden(true);
    pista.set_hidden(false);
    atualizar()?;

    Ok(())
}

// Muda e seta a tela de escolha de pista para usar como callback
fn mudar_para_escolha() -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let escolher = document.get_element_by_id("escolher").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    let pista = document.get_element_by_id("pista").unwrap().dyn_into::<web_sys::HtmlElement>()?;

    pista.set_hidden(true);
    escolher.set_hidden(false);

    Ok(())
}

// Função que fica atualizando o tempo através do setTimeout e carrega a tela de escolha de pista quando acaba o tempo
fn atualizar() -> Result<(), JsValue> {
    let som_termino = HtmlAudioElement::new_with_src(&format!("data:audio/wav;base64,{}", SOM_TERMINO))?;
    let window = window().expect("no global `window` exists");

    let tempo = pegar_tempo_restante().unwrap();

    if tempo <= 0 {
        let acaba_pista = Closure::wrap(Box::new(|| {
            let _ = mudar_para_escolha();
        }) as Box<dyn FnMut()>);

        let _ = som_termino.play().unwrap().finally(&acaba_pista);
        acaba_pista.forget();
    } else {
        atualizar_tempo_restante(tempo - 1)?;

        let contagem = Closure::wrap(Box::new(|| {
            atualizar().unwrap();
        }) as Box<dyn FnMut()>);

        window.set_timeout_with_callback_and_timeout_and_arguments_0(contagem.as_ref().unchecked_ref(), 850)?;
        contagem.forget();
    }

    Ok(())
}

// Função que muda o tempo restante na tela
fn atualizar_tempo_restante (tempo_restante: i8) -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let texto_pista = document.get_element_by_id("tempoRestante").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    
    texto_pista.set_inner_text(&format!("{}", tempo_restante));
    Ok(())
}

// Função que pega o tempo restante na tela
fn pegar_tempo_restante() -> Result<i8, JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let tempo_restante = document.get_element_by_id("tempoRestante").unwrap().dyn_into::<web_sys::HtmlElement>()?;
    Ok(tempo_restante.inner_text().trim().parse::<i8>().unwrap())
}