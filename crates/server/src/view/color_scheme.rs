//! Переключатель цветовой схемы в шапке: `ColorSchemeSelector` из wingy-hypertext с подписями на языке запроса.
//! Поведение — `wingy_hypertext_web::component::color_scheme_selector`.

use hypertext::Raw;
use hypertext::prelude::*;
use wingy_hypertext::component::color_scheme_selector::ColorSchemeSelector;

use crate::i18n::Locale;

/// Применяет сохранённую тёмную схему до первой отрисовки, чтобы страница не мигала светлой до загрузки WASM.
/// Ключ `localStorage`, значения и класс — как в `wingy_hypertext_web::util::color_scheme`.
const EARLY_APPLY_SCRIPT: &str = "try{var s=localStorage.getItem('color-scheme');\
    if(s==='dark'||(s!=='light'&&matchMedia('(prefers-color-scheme: dark)').matches))\
    document.documentElement.classList.add('wa-dark')}catch(e){}";

pub fn early_apply_script() -> impl Renderable {
    // XSS SAFETY: константа без подстановок.
    rsx! { <script>(Raw::dangerously_create(EARLY_APPLY_SCRIPT))</script> }
}

pub fn selector(locale: Locale) -> impl Renderable {
    rsx! {
        <ColorSchemeSelector
            title=(locale.t("color-scheme"))
            label_light=(locale.t("color-scheme-light"))
            label_dark=(locale.t("color-scheme-dark"))
            label_auto=(locale.t("color-scheme-auto"))
        />
    }
}
