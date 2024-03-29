use leptos::*;
use leptos_router::use_location;

use crate::components::common::components::common_icons::Icon;

const SELECTED_CLASS: &str = "font-semibold justify-start capitalize border border-success text-success rounded-xl w-full flex flex-row gap-2 items-center p-2 hover:bg-base-100";
const UNSELECTED_CLASS: &str =
    "font-normal justify-start capitalize w-full rounded-xl flex flex-row gap-2 items-center p-2 hover:bg-base-100";
const SELECTED_CLASS_MOBILE: &str =
    "flex justify-center border border-success text-success rounded-xl p-2 hover:bg-base-100";
const UNSELECTED_CLASS_MOBILE: &str = "flex justify-center rounded-xl p-2 hover:bg-base-100";

#[allow(non_snake_case)]
#[component]
pub fn Sidebar() -> impl IntoView {
    let location = use_location().pathname;

    let toogle = RwSignal::new(true);
    create_effect(move |_| {
        log::info!("toogle: {}", toogle.get());
    });

    {
        move || {
            view! {
                <div class="flex flex-col min-h-full justify-between">
                    <div class = "p-1 m-1">
                        <div class="flex items-center justify-center px924:justify-start">
                            <div class = "flex gap-2 p-2">
                                <Icon
                                    title = "Jabra".to_string()
                                    size = "w-10 h-10".to_string()
                                />
                                <div class="hidden px924:block">
                                    <div class = "text-base font-bold">Jabra.</div>
                                    <div class = "text-xs text-gray-500">Technology</div>
                                </div>
                            </div>
                        </div>

                        <div class="divider divider-neutral mt-0 mb-0"></div>
                        <div class = "text-xs p-2 hidden px924:block">OVERVIEW</div>

                        <MenuWithSubMenu
                            sub_menu_open = location.get() == "/quote_builder" || location.get() == "/active_quotes"
                            title = "QUOTES".to_string()
                            icon_title = "Quotes".to_string()
                        >
                            <Anchor
                                location = location.get()
                                anchor_url = "/quote_builder".to_string()
                                title = "Quote Builder".to_string()
                                sub_anchor = true
                            />
                            <Anchor
                                location = location.get()
                                anchor_url = "/active_quotes".to_string()
                                title = "Active Quotes".to_string()
                                sub_anchor = true
                            />
                        </MenuWithSubMenu>

                        <MenuWithSubMenu
                            sub_menu_open = location.get() == "/positions" || location.get() == "/trade_history"
                            title = "TRADES".to_string()
                            icon_title = "Trades".to_string()
                        >
                            <Anchor
                                location = location.get()
                                anchor_url = "/positions".to_string()
                                title = "Positions".to_string()
                                sub_anchor = true
                            />
                            <Anchor
                                location = location.get()
                                anchor_url = "/trade_history".to_string()
                                title = "Trade History".to_string()
                                sub_anchor = true
                            />
                        </MenuWithSubMenu>

                        <div class="divider divider-neutral mt-0 mb-0"></div>
                        <div class = "text-xs p-2 hidden px924:block">ACCOUNT</div>

                        <Anchor
                            location = location.get()
                            anchor_url = "/login".to_string()
                            title = "Login".to_string()
                            icon_title = "Login".to_string()
                        />

                        <div class="divider divider-neutral mt-0 mb-0"></div>
                        <div class = "text-xs p-2 hidden px924:block">PLAYGROUND</div>

                        <Anchor
                            location = location.get()
                            anchor_url = "/components".to_string()
                            title = "Components".to_string()
                            icon_title = "Component".to_string()
                        />
                    </div>

                    <div class="p-2 flex justify-center">
                        <crate::components::dark_mode_toggle::DarkModeToggle />
                    </div>
                </div>
            }
        }
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Anchor(
    location: String,
    anchor_url: String,
    title: String,
    #[prop(optional)] sub_anchor: bool,
    #[prop(optional)] icon_title: String,
) -> impl IntoView {
    let is_selected = move |is_selected: bool, mobile: bool| {
        if mobile {
            if is_selected {
                SELECTED_CLASS_MOBILE
            } else {
                UNSELECTED_CLASS_MOBILE
            }
        } else {
            if is_selected {
                SELECTED_CLASS
            } else {
                UNSELECTED_CLASS
            }
        }
    };

    let anchor_url_fn = move || format!("{}", anchor_url.clone());
    let location_fn = move || format!("{}", location.clone());

    {
        move || {
            match sub_anchor {
                true => view! {
                    <a class = {is_selected(location_fn() == {format!("{}", anchor_url_fn())}, false)} href = {format!("{}", anchor_url_fn())}>
                        <span class="text-center">{format!("{}", title.clone())}</span>
                    </a>
                }.into_view(),
                false => view! {
                    <div class="items-center hidden px924:block">
                        <a class = {is_selected(location_fn() == {format!("{}", anchor_url_fn())}, false)} href = {format!("{}", anchor_url_fn())}>
                            <Icon
                                title = icon_title.clone()
                                size = "w-5 h-5".to_string()
                                // size = "22px".to_string()
                            />
                            <span class="text-center">{format!("{}", title.clone())}</span>
                        </a>
                    </div>

                    // ------------------ MOBILE ------------------

                    <div class = "flex flex-col gap-2 px924:hidden">
                        <a class = {is_selected(location_fn() == {format!("{}", anchor_url_fn())}, true)} href = {format!("{}", anchor_url_fn())}>
                            <Icon
                                title = icon_title.clone()
                                size = "w-6 h-6".to_string()
                                // size = "26px".to_string()
                            />
                        </a>
                    </div>
                }.into_view(),
            }
        }
    }
}

#[allow(non_snake_case)]
#[component]
pub fn MenuWithSubMenu(
    sub_menu_open: bool,
    title: String,
    icon_title: String,
    children: ChildrenFn,
) -> impl IntoView {
    let is_selected_collapse = move |is_selected_collapse: bool| {
        if is_selected_collapse {
            SELECTED_CLASS_MOBILE
        } else {
            UNSELECTED_CLASS_MOBILE
        }
    };

    {
        move || {
            view! {
                <details class="collapse collapse-arrow hidden px924:block" open={move || sub_menu_open}>
                    <summary class="collapse-title text-md p-2 hover:bg-base-100">
                    <div class="flex flex-row gap-2 items-center">
                        <Icon
                            title = icon_title.clone()
                            size = "w-5 h-5".to_string()
                            // size = "22px".to_string()
                        />
                        <span>{format!("{}", title.clone())}</span>
                    </div>
                    </summary>
                    <div class="collapse-content text-xs">
                        {children()}
                    </div>
                </details>

                // ------------------ MOBILE ------------------

                <div class = "flex flex-col gap-2 px924:hidden">
                    <div class="dropdown dropdown-right">
                        <div tabindex="0" role="button" class = {move || is_selected_collapse(sub_menu_open)}>
                            <Icon
                                title = icon_title.clone()
                                size = "w-6 h-6".to_string()
                                // size = "26px".to_string()
                            />
                        </div>
                        <div class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">
                            {children()}
                        </div>
                    </div>
                </div>
            }
        }
    }
}
