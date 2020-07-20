extern crate wasm_bindgen;
extern crate web_sys;

mod pistas;
mod sons;

use wasm_bindgen::prelude:: { Closure, wasm_bindgen, JsValue };
use web_sys:: { Window, window, Document, HtmlElement, HtmlInputElement, HtmlSelectElement, HtmlAudioElement };
use wasm_bindgen::JsCast;
use pistas::LIVROS;
use sons:: { SOM_INICIO, SOM_TERMINO };

// Importa a função main para ser executada quando a página abre
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window: Window = window().unwrap();
    let document: Document = window.document().unwrap();
    let escolher: HtmlElement = document.get_element_by_id("escolher").unwrap().dyn_into::<HtmlElement>()?;
    let pista: HtmlElement = document.get_element_by_id("pista").unwrap().dyn_into::<HtmlElement>()?;
    let ver_pista: HtmlElement = document.get_element_by_id("verPista").unwrap().dyn_into::<HtmlElement>()?;

    escolher.set_hidden(false);
    pista.set_hidden(true);

    // Callback para o clique do botão
    let btn_click = Closure::wrap(Box::new(move || {
        usar_pista().unwrap();
    }) as Box<dyn FnMut()>);

    ver_pista.set_onclick(Some(btn_click.as_ref().unchecked_ref()));
    btn_click.forget();

    Ok(())
}

// Função executada quando clica no botão de ver pista
fn usar_pista() -> Result<(), JsValue> {
    let window: Window = window().unwrap();
    let document: Document = window.document().unwrap();
    let numero_pista: HtmlInputElement = document.get_element_by_id("numeroPista").unwrap().dyn_into::<HtmlInputElement>()?;
    
    match numero_pista.value().trim().parse::<usize>() {
        Ok(pista_valor) => {
            let numero_livro: HtmlSelectElement = document.get_element_by_id("numeroLivro").unwrap().dyn_into::<HtmlSelectElement>()?;
            match numero_livro.value().trim().parse::<usize>() {
                Ok(livro_valor) =>{
                    if livro_valor < 3 {
                        if pista_valor > 0 && pista_valor < 281 {
                            let som_inicio: HtmlAudioElement = HtmlAudioElement::new_with_src(&format!("data:audio/wav;base64,{}", SOM_INICIO))?;
                            let texto_pista: HtmlElement = document.get_element_by_id("textoPista").unwrap().dyn_into::<HtmlElement>()?;
                            let tempo_restante: HtmlElement = document.get_element_by_id("tempoRestante").unwrap().dyn_into::<HtmlElement>()?;
                            let escolher: HtmlElement = document.get_element_by_id("escolher").unwrap().dyn_into::<HtmlElement>()?;
                            let pista: HtmlElement = document.get_element_by_id("pista").unwrap().dyn_into::<HtmlElement>()?;

                            let _ = som_inicio.play().unwrap();
                            texto_pista.set_inner_text(LIVROS[livro_valor][pista_valor - 1]);
                            escolher.set_hidden(true);
                            pista.set_hidden(false);
                            tempo_restante.set_inner_text(&format!("{}", 30));

                            atualizar()?;
                        } else {
                            window.alert_with_message("Digite um número entre 1 e 280.")?;
                        }
                    } else {
                        window.alert_with_message("Escolha um dos 3 livros.")?;
                    }
                }
                Err(_) => {
                    window.alert_with_message("Escolha um dos 3 livros.")?;
                }
            }  
        }
        Err(_) => {
            window.alert_with_message("Digite um número entre 1 e 280.")?;
        }
    }

    Ok(())
}

// Função que fica atualizando o tempo através do setTimeout e carrega a tela de escolha de pista quando acaba o tempo
fn atualizar() -> Result<(), JsValue> {
    let window: Window = window().unwrap();
    let document: Document = window.document().unwrap();
    let tempo_restante: HtmlElement = document.get_element_by_id("tempoRestante").unwrap().dyn_into::<HtmlElement>()?;

    let tempo = tempo_restante.inner_text().trim().parse::<i8>().unwrap();

    if tempo <= 0 {
        let som_termino: HtmlAudioElement = HtmlAudioElement::new_with_src(&format!("data:audio/wav;base64,{}", SOM_TERMINO))?;
        let escolher: HtmlElement = document.get_element_by_id("escolher").unwrap().dyn_into::<HtmlElement>()?;
        let pista: HtmlElement = document.get_element_by_id("pista").unwrap().dyn_into::<HtmlElement>()?;

        let _ = som_termino.play().unwrap();
        escolher.set_hidden(false);
        pista.set_hidden(true);
    } else {
        let tempo_restante: HtmlElement = document.get_element_by_id("tempoRestante").unwrap().dyn_into::<HtmlElement>()?;
        tempo_restante.set_inner_text(&format!("{}", tempo - 1));

        let contagem = Closure::wrap(Box::new(|| {
            atualizar().unwrap();
        }) as Box<dyn FnMut()>);

        window.set_timeout_with_callback_and_timeout_and_arguments_0(contagem.as_ref().unchecked_ref(), 850)?;
        contagem.forget();
    }

    Ok(())
}