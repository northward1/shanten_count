use std::{str::FromStr, sync::OnceLock};

use rustc_hash::FxHashMap;
use shanten_count::shanten::{Hand, JihaiHand, SuuhaiHand};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/>},
        Route::ImageGenerator => html! {<ImageGenerator/>},
        Route::ShantenCaluculator => html! {<ShantenCaluculator/>},
    }
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <>
            <a href="/image-generator">{"牌姿生成"}</a> <br/>
            <a href="/shanten-calculator">{"シャンテン数計算"}</a>
        </>
    }
}

#[function_component(ImageGenerator)]
fn image_generator() -> Html {
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_change = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();

            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    html! {
        <>
            <input
                onchange={on_change}
                value={input_value.clone()}
            />
            <br/>
            <br/>
            <div>
            {parse_input(&input_value.clone()).iter().map(|p|
                html! {<img src={p.clone()} width=90 height=120/>}
            ).collect::<Html>()}
            </div>
        </>
    }
}

fn parse_input(input: &str) -> Vec<String> {
    let chars = input.chars().collect::<Vec<_>>();
    let length = chars.len();

    let mut tiles = vec![];

    for i in 0..length {
        if chars[i] == '?' {
            tiles.push("images/blank.png".to_string());
        } else if chars[i] == '_' {
            tiles.push("images/back.png".to_string());
        } else {
            let c = chars[i];

            if ('0'..='9').contains(&c) {
                let v = c.to_digit(10).unwrap() as u8;

                let tile_type = (i + 1..length)
                    .map(|j| chars[j])
                    .filter(|&c| c == 'm' || c == 'p' || c == 's' || c == 'z')
                    .next();

                if let Some(tile_type) = tile_type {
                    match tile_type {
                        'm' => {
                            tiles.push(format!("images/manzu{}.png", v));
                        }
                        's' => {
                            tiles.push(format!("images/sozu{}.png", v));
                        }
                        'p' => {
                            tiles.push(format!("images/pinzu{}.png", v));
                        }
                        'z' => {
                            tiles.push(format!("images/jihai{}.png", v));
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    return tiles;
}

static SUUHAI_DICT: OnceLock<FxHashMap<(SuuhaiHand, u8), u8>> = OnceLock::new();
static JIHAI_DICT: OnceLock<FxHashMap<(JihaiHand, u8), u8>> = OnceLock::new();

fn shanten_info_text(input: &str) -> Html {
    SUUHAI_DICT.get_or_init(SuuhaiHand::calc_shanten_to_all_partly_pattern);
    JIHAI_DICT.get_or_init(JihaiHand::calc_shanten_to_all_partly_pattern);

    let shanten_count_text;
    let hand = Hand::from_str(input);

    if hand.is_ok() {
        let hand = hand.unwrap();

        let standard_shanten =
            hand.shanten_standard(&SUUHAI_DICT.get().unwrap(), &JIHAI_DICT.get().unwrap());
        let chiitoitsu_shanten = hand.shanten_chiitoitsu();
        let kokushi_shanten = hand.shanten_kokushimusou();

        shanten_count_text = format!(
            "一般形: {}, 七対子: {}, 国士無双: {}",
            standard_shanten, chiitoitsu_shanten, kokushi_shanten
        );
    } else {
        shanten_count_text = format!("Failed to parse.");
    }

    html! {<>{shanten_count_text}</>}
}

#[function_component(ShantenCaluculator)]
fn shanten_calculator() -> Html {
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_change = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();

            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    html! {
        <>
            <input
                onchange={on_change}
                value={input_value.clone()}
            />
            <br/>
            {shanten_info_text(&input_value)}
            <br/>
            <div>
            {parse_input(&input_value).iter().map(|p|
                html! {<img src={p.clone()} width=90 height=120/>}
            ).collect::<Html>()}
            </div>
        </>
    }
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[at("/image-generator")]
    ImageGenerator,
    #[at("/shanten-calculator")]
    ShantenCaluculator,
    #[at("/")]
    Home,
}

fn main() {
    yew::Renderer::<App>::new().render();
}
