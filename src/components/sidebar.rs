use leptos::*;
use leptos_router::use_location;

const SELECTED_CLASS: &str = "font-semibold justify-start capitalize border border-success text-success rounded-xl w-full flex flex-row gap-2 items-center p-2";
const UNSELECTED_CLASS: &str =
    "font-normal justify-start capitalize w-full rounded-xl flex flex-row gap-2 items-center p-2";
const SELECTED_CLASS_MOBILE: &str =
    "flex justify-center border border-success text-success rounded-xl p-2";
const UNSELECTED_CLASS_MOBILE: &str = "flex justify-center rounded-xl p-2";

#[component]
pub fn Sidebar() -> impl IntoView {
    let location = use_location().pathname;

    view! {
        <div class = "p-1 m-1">
            <div class="flex items-center justify-center md:justify-start">
                <div class = "flex flex wrap gap-2 p-2">
                    <Icon
                        title = "Jabra".to_string()
                        size = "w-10 h-10".to_string()
                    />
                    <div class="hidden md:block">
                        <div class = "text-base font-bold">Jabra.</div>
                        <div class = "text-xs text-gray-500">Technology</div>
                    </div>
                </div>
            </div>

            <div class="divider divider-neutral mt-0 mb-0"></div>
            <div class = "text-xs p-2 hidden md:block">OVERVIEW</div>

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
                    icon_title = None
                />
                <Anchor
                    location = location.get()
                    anchor_url = "/active_quotes".to_string()
                    title = "Active Quotes".to_string()
                    sub_anchor = true
                    icon_title = None
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
                    icon_title = None
                />
                <Anchor
                    location = location.get()
                    anchor_url = "/trade_history".to_string()
                    title = "Trade History".to_string()
                    sub_anchor = true
                    icon_title = None
                />
            </MenuWithSubMenu>

            <div class="divider divider-neutral mt-0 mb-0"></div>
            <div class = "text-xs p-2 hidden md:block">ACCOUNT</div>

            <Anchor
                location = location.get()
                anchor_url = "/settings".to_string()
                title = "Settings".to_string()
                sub_anchor = false
                icon_title = Some("Settings".to_string())
            />
            <Anchor
                location = location.get()
                anchor_url = "/login".to_string()
                title = "Login".to_string()
                sub_anchor = false
                icon_title = Some("Login".to_string())
            />

        </div>
    }
}

#[island]
pub fn Anchor(
    location: String,
    anchor_url: String,
    title: String,
    sub_anchor: bool,
    icon_title: Option<String>,
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
                    <a class = {is_selected(location_fn() == {format!("{}", anchor_url_fn())}, false)} class="hover:bg-base-100" href = {format!("{}", anchor_url_fn())}>
                        <span class="text-center">{format!("{}", title.clone())}</span>
                    </a>
                }.into_view(),
                false => view! {
                    <div class="flex items-center hidden md:block">
                        <a class = {is_selected(location_fn() == {format!("{}", anchor_url_fn())}, false)} class="hover:bg-base-100" href = {format!("{}", anchor_url_fn())}>
                            <Icon
                                title = icon_title.clone().unwrap()
                                size = "w-5 h-5".to_string()
                                // size = "22px".to_string()
                            />
                            <span class="text-center">{format!("{}", title.clone())}</span>
                        </a>
                    </div>

                    // ------------------ MOBILE ------------------

                    <div class = "flex flex-col gap-2 block md:hidden">
                        <a class = {is_selected(location_fn() == {format!("{}", anchor_url_fn())}, true)} class="hover:bg-base-100" href = {format!("{}", anchor_url_fn())}>
                            <Icon
                                title = icon_title.clone().unwrap()
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

    view! {
        <details class="collapse collapse-arrow hidden md:block" open={move || sub_menu_open}>
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

        <div class = "flex flex-col gap-2 block md:hidden">
            <div class="dropdown dropdown-right">
                <div tabindex="0" role="button" class="hover:bg-base-100" class = {move || is_selected_collapse(sub_menu_open)}>
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

#[component]
pub fn Icon(title: String, size: String) -> impl IntoView {
    match title.as_str() {
        "Dashboard" => view! {
            // <phosphor_leptos::House size=format!("{}", size.clone())/>
            <span>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class={format!("{}", size.clone())}>
                <path fill-rule="evenodd" d="M4.5 2A1.5 1.5 0 003 3.5v13A1.5 1.5 0 004.5 18h11a1.5 1.5 0 001.5-1.5V7.621a1.5 1.5 0 00-.44-1.06l-4.12-4.122A1.5 1.5 0 0011.378 2H4.5zm2.25 8.5a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-6.5zm0 3a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-6.5z" clip-rule="evenodd" />
                </svg>
            </span>
        }.into_view(),
        "Quotes" => view! {
            // <phosphor_leptos::Note size=format!("{}", size.clone())/>
            <span>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class={format!("{}", size.clone())}>
                <path fill-rule="evenodd" d="M4.5 2A1.5 1.5 0 003 3.5v13A1.5 1.5 0 004.5 18h11a1.5 1.5 0 001.5-1.5V7.621a1.5 1.5 0 00-.44-1.06l-4.12-4.122A1.5 1.5 0 0011.378 2H4.5zm2.25 8.5a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-6.5zm0 3a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-6.5z" clip-rule="evenodd" />
                </svg>
            </span>
        }.into_view(),
        "Trades" => view! {
            // <phosphor_leptos::PresentationChart size=format!("{}", size.clone())/>
            <span>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class={format!("{}", size.clone())}>
                <path fill-rule="evenodd" d="M1 2.75A.75.75 0 011.75 2h16.5a.75.75 0 010 1.5H18v8.75A2.75 2.75 0 0115.25 15h-1.072l.798 3.06a.75.75 0 01-1.452.38L13.41 18H6.59l-.114.44a.75.75 0 01-1.452-.38L5.823 15H4.75A2.75 2.75 0 012 12.25V3.5h-.25A.75.75 0 011 2.75zM7.373 15l-.391 1.5h6.037l-.392-1.5H7.373zm7.49-8.931a.75.75 0 01-.175 1.046 19.326 19.326 0 00-3.398 3.098.75.75 0 01-1.097.04L8.5 8.561l-2.22 2.22A.75.75 0 115.22 9.72l2.75-2.75a.75.75 0 011.06 0l1.664 1.663a20.786 20.786 0 013.122-2.74.75.75 0 011.046.176z" clip-rule="evenodd" />
                </svg>
            </span>
        }.into_view(),
        "Settings" => view! {
            // <phosphor_leptos::Gear size=format!("{}", size.clone())/>
            <span>
                <svg class={format!("{}", size.clone())} fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                </svg>
            </span>
        }.into_view(),
        "Login" => view! {
            // <phosphor_leptos::SignIn size=format!("{}", size.clone())/>
            <span>
                <svg xmlns="http://www.w3.org/2000/svg" class={format!("{}", size.clone())} fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15m3 0l3-3m0 0l-3-3m3 3H9" />
                </svg>
            </span>
        }.into_view(),
        "Logout" => view! {
            // <phosphor_leptos::SignOut size=format!("{}", size.clone())/>
            <span>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class={format!("{}", size.clone())}>
                    <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15M12 9l-3 3m0 0l3 3m-3-3h12.75" />
                </svg>
            </span>
        }.into_view(),
        "Jabra" => view! {
            <svg class={format!("{}", size.clone())} xmlns="http://www.w3.org/2000/svg" width="58" height="58" viewBox="0 0 58 58" fill="currentColor">
                <path id="Union" d="M35.2282 8H19.1568L21.0518 9.25316C21.5791 9.60182 22.0033 10.0853 22.2804 10.6535C22.5751 11.2575 22.7282 11.9207 22.7282 12.5928V39.6964C22.7282 42.739 21.118 45.5543 18.4955 47.0969L15.1389 49.0714H22.0723C23.9694 49.0714 25.8239 48.5084 27.4008 47.4537C30.0603 45.6749 31.6568 42.6864 31.6568 39.487V12.6519C31.6568 11.9418 31.8256 11.2418 32.1493 10.6098C32.4262 10.0693 32.8358 9.60799 33.3398 9.26924L35.2282 8Z" fill="currentColor"/>
                <path id="Union_2" d="M10.2282 41.9285C13.1869 41.9285 15.5853 39.5301 15.5853 36.5714C15.5853 33.6127 13.1869 31.2143 10.2282 31.2143C8.63502 31.2143 7.20428 31.9097 6.22299 33.0136C6.21879 33.0091 6.2146 33.0045 6.21042 33C4.12708 35.0833 1.65685 40.9464 8.44256 47.7321C7.47374 46.348 6.14448 43.2905 7.9748 41.433C8.65979 41.751 9.42326 41.9285 10.2282 41.9285Z" fill="currentColor"/>
                <path d="M38.8216 28.5357H44.6251C49.0894 28.5357 54.0001 33.3274 54.0001 38.8035C54.0001 44.2797 49.5358 49.0714 44.6251 49.0714H38.8216C44.5358 46.6756 45.6668 41.2279 45.518 38.8035C45.6668 36.3792 44.5358 30.9315 38.8216 28.5357Z" fill="currentColor"/>
                <path d="M38.8215 8H44.6251C49.0894 8 54.0001 12.7917 54.0001 18.2678C54.0001 23.744 49.5358 28.5357 44.6251 28.5357H38.8216C44.5358 26.1399 45.6668 20.6922 45.518 18.2678C45.6668 15.8435 44.5358 10.3958 38.8215 8Z" fill="currentColor"/>
            </svg>
        }.into_view(),
        _ => view! {
            <div></div>
        }.into_view(),
    }
}
