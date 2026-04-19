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

static NUM_IMAGES: u32 = 4;
struct HeroState {
    image_index: u32,
    image_dot_refs: Vec::<(Element, EventListener)>,
}
impl HeroState {
    fn new() -> Self {
        Self {
            image_index: NUM_IMAGES - 1,
            image_dot_refs: Vec::new(),
        }
    }

    /// Returns the index of the next image using 1-based indexing
    fn next_image(&mut self) {
        self.image_index = (self.image_index + 1) % NUM_IMAGES;

        self.set_active_image(self.image_index);
    }

    fn set_active_image(&mut self, i: u32) {
        let document = web_sys::window().unwrap().document().unwrap();
        let image: HtmlImageElement = document.query_selector(".hero__image")
            .unwrap()
            .expect("will exist")
            .unchecked_into();

        self.image_index = i % NUM_IMAGES;
        image.set_src(format!("./resources/hero/hero-{}.png", self.image_index + 1).as_str());

        self.image_dot_refs.iter()
                    .enumerate()
                    .for_each(|(j, (dot, _)): (usize, &(Element, EventListener))| dot.set_attribute("aria-selected", if (j as u32) == i { "true" } else { "false" }).unwrap() );
    }
}

struct HomePage {
    state: Rc<RefCell<HeroState>>,
    hero_poll: Rc<RefCell<Interval>>,
}
impl HomePage {
    fn new() -> Result<Self, JsValue> {
        let state = Rc::new(RefCell::new(HeroState::new()));

        let poll_state = Rc::clone(&state);

        let document = web_sys::window().unwrap().document().unwrap();

        document.body().unwrap().set_inner_html(include_str!("../pages/home.html"));

        HomePage::init_hero_quote(&document)?;

        let mut hero_dot_html = String::new();
        // create image toggle buttons
        for i in 1..= NUM_IMAGES {
            hero_dot_html.push_str(r#"<button class="hero__dot" role="tab" aria-selected="false"></button>"#);
        }
        document.query_selector(".hero__dots")
            .unwrap()
            .expect("will exist")
            .set_inner_html(&hero_dot_html);

        let state_ref = state.clone();
        let hero_poll = Rc::new(RefCell::new(Interval::new(6000, move || {
            state_ref.borrow_mut().next_image();
        })));

        let hero_dots = document.get_elements_by_class_name("hero__dot");

        for i in 0..hero_dots.length() {
            let hero_dot = hero_dots.get_with_index(i).unwrap();
            let state_ref = state.clone();
            let curr_i = i.clone();
            let hero_poll_ref = hero_poll.clone();

            let event_listener = EventListener::new(&hero_dot, "click", move |_| {
                state_ref.borrow_mut().set_active_image(curr_i);

                let poll_state_ref = state_ref.clone();
                drop(hero_poll_ref.replace(Interval::new(6000, move || {
                    poll_state_ref.borrow_mut().next_image();
                })));
            });
            state.borrow_mut().image_dot_refs.push((hero_dot, event_listener))
        }

        state.borrow_mut().set_active_image(0);

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

struct PortfolioPage;
impl PortfolioPage {
    fn new() -> Result<Self, JsValue> {
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

struct RunPage;
impl RunPage {
    fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        document.body().unwrap().set_inner_html(include_str!("../pages/run.html"));
        Self
    }
}
        

enum PageRoute {
    Home(HomePage),
    Portfolio(PortfolioPage),
    Blog(BlogPage),
    RunLog(RunPage),
    NotFound,
}

impl TryFrom<&str> for PageRoute {
    type Error = JsValue;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "/" => Ok(PageRoute::Home(HomePage::new()?)),
            "/portfolio" => Ok(PageRoute::Portfolio(PortfolioPage::new()?)),
            "/blogs" => Ok(PageRoute::Blog(BlogPage::new())),
            "/run-log" => Ok(PageRoute::RunLog(RunPage::new())),
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

        let route = Rc::new(RefCell::new(
                window.location()
                    .hash()
                    .unwrap()
                    .strip_prefix("#")
                    .unwrap_or("/")
                    .try_into()?));

        let route_change_route = route.clone();
        let route_change_event = EventListener::new(&window.clone(), "hashchange", move |_event| {
            let new_route: PageRoute = web_sys::window()
                .expect("no global `window` exists")
                .location()
                .hash()
                .unwrap()
                .strip_prefix("#")
                .unwrap_or("/")
                .try_into()
                .unwrap();
            drop(route_change_route.replace(new_route));
        });

        Ok(Self {
            route,
            route_change_event,
        })
    }
}

// Called when the Wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    Ok(())
}
