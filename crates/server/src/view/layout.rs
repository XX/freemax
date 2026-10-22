use hypertext::prelude::*;
use iconic::fontawesome;
use wingy_hypertext::class::{CLUSTER, FLANK, ICON, PAGE_MENU_NAV, SPLIT};
use wingy_hypertext::layout::page::{Page, PageBody, PageMenu, PageNavigationToggle};

use super::{View, color_scheme, language};
use crate::i18n::Locale;

pub const APP_NAME: &str = "Freemax";

/// Элемент, в который htmx подставляет содержимое при навигации.
pub const MAIN_TARGET: &str = ".main-content";

const NAV_ID: &str = "main-nav";

/// Разделы навигации: идентификатор сообщения с названием и путь.
pub const NAV_ITEMS: [(&str, &str); 3] = [
    ("nav-listings", "/"),
    ("nav-circles", "/circles"),
    ("nav-about", "/about"),
];

fn title_text(view: &View) -> String {
    format!("{} — {APP_NAME}", view.title)
}

/// Полная страница.
pub fn document<'a>(view: &'a View, path: &'a str, locale: Locale) -> impl Renderable + 'a {
    rsx! {
        <!DOCTYPE html>
        <html lang=(locale.code())>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>(title_text(view))</title>
                (color_scheme::early_apply_script())
                <link rel="stylesheet" href="/webassets/main.css">
                <script src="/webassets/vendor/htmx/2.0.8/htmx.min.js"></script>
                <script type="module" src="/webassets/main.js"></script>
            </head>
            <body>
                <Page navigation=true>
                    <header class=SPLIT>
                        <div class=CLUSTER>
                            <PageNavigationToggle/>
                            <a class="brand" href="/">
                                <span class=("brand-icon ", ICON)>(fontawesome::solid::Heart)</span>
                                <strong>(APP_NAME)</strong>
                            </a>
                        </div>
                        <div class=CLUSTER>
                            (language::selector(locale))
                            (color_scheme::selector(locale))
                        </div>
                    </header>
                    <PageBody>
                        <PageMenu>
                            (nav(path, locale, false))
                        </PageMenu>
                        <main class="main-content">
                            (&view.content)
                        </main>
                    </PageBody>
                    <footer>
                        <small>(locale.t("footer-tagline"))</small>
                    </footer>
                </Page>
            </body>
        </html>
    }
}

/// Фрагмент для htmx: содержимое основной области, заголовок документа и навигация (out-of-band).
pub fn fragment<'a>(view: &'a View, path: &'a str, locale: Locale) -> impl Renderable + 'a {
    rsx! {
        <title>(title_text(view))</title>
        (&view.content)
        (nav(path, locale, true))
    }
}

/// Навигация по разделам; активный раздел определяется по пути запроса.
fn nav(path: &str, locale: Locale, oob: bool) -> impl Renderable + '_ {
    rsx! {
        <nav id=NAV_ID class=PAGE_MENU_NAV hx-swap-oob=[oob.then_some("true")]>
            @for (label_id, href) in NAV_ITEMS {
                <a
                    class=FLANK
                    href=(href)
                    hx-get=(href)
                    hx-target=MAIN_TARGET
                    hx-swap="innerHTML"
                    hx-push-url="true"
                    aria-current=[is_current(path, href).then_some("page")]
                >
                    <span>(locale.t(label_id))</span>
                </a>
            }
        </nav>
    }
}

fn is_current(path: &str, href: &str) -> bool {
    if href == "/" {
        path == "/"
    } else {
        path == href || path.strip_prefix(href).is_some_and(|rest| rest.starts_with('/'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_section() {
        assert!(is_current("/", "/"));
        assert!(!is_current("/circles", "/"));
        assert!(is_current("/circles", "/circles"));
        assert!(is_current("/circles/42", "/circles"));
        assert!(!is_current("/circlesx", "/circles"));
    }
}
