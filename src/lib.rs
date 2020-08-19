#![allow(non_snake_case)]

use std::cell::RefCell;
use std::collections::VecDeque;

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//https://riptutorial.com/rust/example/25994/thread-local-objects

struct AppItem {
    current: u32
}

impl AppItem {
    fn new(current: u32) -> AppItem {
        AppItem {
            current
        }
    }
}

impl Drop for AppItem {
    fn drop(&mut self) {
        let message = format!("AppItem Drop = {}", self.current);
        consoleLog(message);
    }
}

struct AppState {
    counter: u32,
    list: VecDeque<AppItem>
}

impl AppState {
    fn new() -> AppState {
        AppState {
            counter: 1,
            list: VecDeque::new()
        }
    }

    fn getNextId(&mut self) -> u32 {
        self.counter += 2;
        let item = AppItem::new(self.counter);
        self.list.push_back(item);

        if self.list.len() > 5 {
            self.list.pop_front();
        }

        self.counter
    }
}

thread_local! {
    static appState: RefCell<AppState> = RefCell::new(AppState::new());
}

#[wasm_bindgen]
extern {
    fn alertConsole(s: &str);
    fn callFromWasm(message: &str);
    fn consoleLog(message: String);
}

#[wasm_bindgen]
//pub fn greet(message: &'static str) {
pub fn greet(message: &str, mes: &str) -> String {
    callFromWasm("Zaczynam alertowanie");
    alertConsole(&format!("pppp3 - Hello, {} == {}", message, mes));
    callFromWasm("Kończę alertowanie");

    "aaaaa kkklops".into()
}


#[wasm_bindgen]
pub fn getNextId() -> u32 {
    let nextId = appState.with(|state| {
        let mut state = state.borrow_mut();
        state.getNextId()
    });

    nextId
}