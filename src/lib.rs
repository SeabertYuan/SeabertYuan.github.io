use wasm_bindgen::prelude::*;
use gloo::{
    events::EventListener,
    timers::callback::Interval,
};
use web_sys::{HtmlImageElement, HtmlElement, Element};

use std::rc::Rc;
use std::cell::RefCell;

// use futures::prelude::*;

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

const ROUTES: [&str; 3] = [
    "#/blogs",
    "#/portfolio",
    "#/run-log",
];

struct HomeState {
    image_index: usize,
}
impl HomeState {
    fn new() -> Self {
        Self {
            image_index: 3,
        }
    }

    /// Returns the index of the next image using 1-based indexing
    fn next_image(&mut self) -> usize {
        // TODO: 4 is a magic number
        let next_index = (self.image_index + 1) % 4;
        self.image_index = next_index;
        self.image_index + 1
    }
}

struct HomePage {
    state: Rc<RefCell<HomeState>>,
    hero_poll: Interval,
}
impl HomePage {
    fn new() -> Result<Self, JsValue> {
        log("constructing home page");
        let state = Rc::new(RefCell::new(HomeState::new()));

        let poll_state = Rc::clone(&state);

        let document = web_sys::window().unwrap().document().unwrap();
        document.body().unwrap().set_inner_html(include_str!("../pages/home.html"));
        Element::unchecked_into::<HtmlElement>(document.document_element().unwrap())
            .style()
            .set_css_text("background: linear-gradient(to bottom, #ffffff 50%, #16161d 50%); background-attachment: fixed;");
        HomePage::init_hero_quote(&document)?;

        // show initial image
        Element::unchecked_into::<HtmlImageElement>(document.query_selector(".hero-image")?
            .expect("exists in the home page")).set_src(
            format!("./resources/hero/hero-{}.png", poll_state.borrow_mut().next_image()).as_str()
            );

        let hero_poll = Interval::new(6000, move || {
            log("polled");
            let document = web_sys::window().unwrap().document().unwrap();
            let image: HtmlImageElement = document.query_selector(".hero-image")
                .unwrap()
                .expect("will exist")
                .unchecked_into();
            image.set_src(format!("./resources/hero/hero-{}.png", poll_state.borrow_mut().next_image()).as_str());
        });

        Ok(Self {
            state,
            hero_poll,
        })
    }

    fn init_hero_quote(document: &web_sys::Document) -> Result<(), JsValue> {
        let subtitle = document.query_selector(".hero-subtitle")?.expect("subtitle will exist");
        let rand: usize = getrandom::u32().map_err(|e| JsValue::from_str(e.to_string().as_str()))? as usize;
        let i = rand % QUOTES.len();
        subtitle.set_text_content(Some(QUOTES[i]));
        Ok(())
    }
}
impl Drop for HomePage {
    fn drop(&mut self) {
        log("dropped home page");
    }
}

struct PortfolioPage;
impl PortfolioPage {
    fn new() -> Result<Self, JsValue> {
        log("constructing portfolio page");
        let document = web_sys::window().unwrap().document().unwrap();
        document.body().unwrap().set_inner_html(include_str!("../pages/portfolio.html"));
        Ok(Self)
    }
}

struct BlogPage;
impl BlogPage {
    fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        document.body().unwrap().set_inner_html(include_str!("../pages/blog.html"));
        Self
    }
}

enum PageRoute {
    Home(HomePage),
    Portfolio(PortfolioPage),
    Blog(BlogPage),
    RunLog,
    NotFound,
}

impl TryFrom<&str> for PageRoute {
    type Error = JsValue;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "/" => Ok(PageRoute::Home(HomePage::new()?)),
            "/portfolio" => Ok(PageRoute::Portfolio(PortfolioPage::new()?)),
            "/blogs" => Ok(PageRoute::Blog(BlogPage::new())),
            "/run-log" => Ok(PageRoute::RunLog),
            _ => Ok(PageRoute::NotFound),
        }
    }
}

#[wasm_bindgen]
struct App {
    route: Rc<RefCell<PageRoute>>,
    route_change_event: EventListener,
}
#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Self, JsValue> {
        let window = web_sys::window().expect("no global 'window' exists");

        let route = Rc::new(RefCell::new(PageRoute::Home(HomePage::new()?)));

        let route_change_route = route.clone();
        let route_change_event = EventListener::new(&window.clone(), "hashchange", move |_event| {
            log("new route detected");
            let new_route: PageRoute = web_sys::window()
                .expect("no global `window` exists")
                .location()
                .hash()
                .unwrap()
                .strip_prefix("#")
                .unwrap_or("/")
                .try_into()
                .unwrap();
            route_change_route.replace(new_route);
            log("successfully rerouted");
        });

        log("built app");

        Ok(Self {
            route,
            route_change_event,
        })
    }
}
impl Drop for App {
    fn drop (&mut self) {
        log("dropped app");
    }
}

// Called when the Wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    Ok(())
}


#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
