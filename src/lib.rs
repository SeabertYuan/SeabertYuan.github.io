use wasm_bindgen::prelude::*;
use gloo::events::EventListener;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const QUOTES: [&str; 6] = [
    "so... the weather's nice?",
    "live life without regrets.",
    "two broken hearts won't mend each other.",
    "1 Corinthians 13:4 | Love is patient, love is kind. It does not envy, it does not boast, it is not proud.",
    "home-made",
    "the race does not end after you cross the finish line.",
];

// Called when the Wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    init_nav_burger(&document)?.forget();

    init_hero_quote(&document)?;

    Ok(())
}

fn init_nav_burger(document: &web_sys::Document) -> Result<EventListener, JsValue> {
    let nav_links = document.get_element_by_id("navLinks").expect("navLinks will exist");
    let burger = document.get_element_by_id("navBurger").expect("navLinks will exist");
    Ok(EventListener::new(&burger.clone(), "click", move |_event| {
        nav_links.class_list().toggle("open");

        burger.class_list().toggle("toggle");
    }))
}

fn init_hero_quote(document: &web_sys::Document) -> Result<(), JsValue> {
    let subtitle = document.get_element_by_id("mainSubtitle").expect("subtitle will exist");
    let rand: usize = getrandom::u32().map_err(|e| JsValue::from_str(e.to_string().as_str()))? as usize;
    let i = rand % QUOTES.len();
    subtitle.set_text_content(Some(QUOTES[i]));
    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
