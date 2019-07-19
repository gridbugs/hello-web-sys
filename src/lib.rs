use js_sys::Function;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent, MouseEvent, WheelEvent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document
        .get_element_by_id("content")
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_inner_text("Hello, World!");
    let mut count = 0;
    let handle_keydown = Closure::wrap(Box::new(move |event: JsValue| {
        let keyboard_event = event.unchecked_ref::<KeyboardEvent>();
        console_log!(
            "keyboard event {} {}",
            keyboard_event.key_code(),
            keyboard_event.shift_key()
        );
    }) as Box<dyn FnMut(JsValue)>);
    window
        .add_event_listener_with_callback("keydown", handle_keydown.as_ref().unchecked_ref())
        .unwrap();
    handle_keydown.forget();
    let handle_mouse = Closure::wrap(Box::new(move |event: JsValue| {
        let mouse_event = event.unchecked_ref::<MouseEvent>();
        console_log!(
            "mouse event {} {} {}",
            mouse_event.buttons(),
            mouse_event.client_x(),
            mouse_event.client_y(),
        );
    }) as Box<dyn FnMut(JsValue)>);
    window
        .add_event_listener_with_callback("mousemove", handle_mouse.as_ref().unchecked_ref())
        .unwrap();
    window
        .add_event_listener_with_callback("mousedown", handle_mouse.as_ref().unchecked_ref())
        .unwrap();
    window
        .add_event_listener_with_callback("mouseup", handle_mouse.as_ref().unchecked_ref())
        .unwrap();
    handle_mouse.forget();
    let handle_wheel = Closure::wrap(Box::new(move |event: JsValue| {
        let wheel_event = event.unchecked_ref::<WheelEvent>();
        console_log!(
            "wheel event {} {} {} {}",
            wheel_event.delta_x(),
            wheel_event.delta_y(),
            wheel_event.client_x(),
            wheel_event.client_y(),
        );
    }) as Box<dyn FnMut(JsValue)>);
    window
        .add_event_listener_with_callback("wheel", handle_wheel.as_ref().unchecked_ref())
        .unwrap();
    handle_wheel.forget();
    let f: Rc<RefCell<Option<Closure<_>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if count % 60 == 0 {
            console_log!("{}", count);
        }
        count += 1;
        window
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut()>));
    g.borrow()
        .as_ref()
        .unwrap()
        .as_ref()
        .unchecked_ref::<Function>()
        .call0(&JsValue::NULL)
        .unwrap();
    Ok(())
}
