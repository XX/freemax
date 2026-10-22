//! Переключатель языка в шапке. Поведение — в `freemax_client::language`: выбор сохраняется в cookie
//! [`LANG_COOKIE`](crate::i18n::LANG_COOKIE), страница перезагружается.

use hypertext::prelude::*;
use wingy_hypertext::appearance::Appearance::Plain;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::attrs;
use wingy_hypertext::class::SIZE_SMALL;
use wingy_hypertext::component::button::Button;
use wingy_hypertext::component::dropdown::{Dropdown, DropdownItem, DropdownItemLabel, DropdownMenu};
use wingy_hypertext::helper::popup::PopupPlacement::BottomEnd;

use crate::i18n::Locale;

/// Должен совпадать с `SELECTOR_ID` в `freemax_client::language`.
const SELECTOR_ID: &str = "language-selector";

pub fn selector(locale: Locale) -> impl Renderable {
    let label = locale.t("language-selector");
    rsx! {
        <Dropdown id=SELECTOR_ID placement=BottomEnd>
            <Button
                class=SIZE_SMALL
                appearance=Plain
                pill=true
                attrs=(attrs!["title" = &label, "aria-label" = &label])
            >
                (locale.code().to_uppercase())
            </Button>
            <DropdownMenu>
                @for item in Locale::ALL {
                    <DropdownItem checkbox=true checked=(item == locale) value=(item.code())>
                        <DropdownItemLabel><span lang=(item.code())>(item.t("language-name"))</span></DropdownItemLabel>
                    </DropdownItem>
                }
            </DropdownMenu>
        </Dropdown>
    }
}
