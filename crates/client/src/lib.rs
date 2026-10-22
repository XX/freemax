//! Клиентская часть Freemax (WASM): поведение компонентов wingy-hypertext.

use wasm_bindgen::prelude::*;
use wingy_hypertext_web::component::color_scheme_selector::{
    init_color_scheme_selectors, listen_color_scheme_selector,
};
use wingy_hypertext_web::component::dropdown::{init_dropdowns, listen_dropdowns};
use wingy_hypertext_web::layout::drawer::{init_drawers, listen_drawers};
use wingy_hypertext_web::{
    init_page_element, init_scroll_to_anchor, listen_click_actions, listen_page, register_copy_action,
};

pub mod language;

/// Однократная настройка: действия и обработчики событий уровня документа.
#[wasm_bindgen]
pub fn init() {
    register_copy_action();
    listen_color_scheme_selector();
    language::listen_language_selector();
    listen_click_actions();
    listen_dropdowns();
    listen_drawers();
    listen_page();
}

/// Повторная инициализация после каждой подстановки htmx.
#[wasm_bindgen]
pub fn reinit() {
    init_page_element();
    init_scroll_to_anchor();
    init_dropdowns();
    init_color_scheme_selectors();
    init_drawers();
}
