use leptos::*;
use leptos_router::use_location;

#[component]
pub fn Sidebar() -> impl IntoView {
    let is_selected = move |is_selected: bool| {
        if is_selected {
            "font-semibold justify-start capitalize border border-success text-success rounded-xl w-full flex flex-row gap-2 items-center p-2"
        } else {
            "font-normal justify-start capitalize w-full rounded-xl flex flex-row gap-2 items-center p-2"
        }
    };


    let is_selected_collapse = move |is_selected_collapse: bool| {
        if is_selected_collapse {
            "flex justify-center border border-success text-success rounded-xl p-2"
        } else {
            "flex justify-center rounded-xl p-2"
        }
    };

    let location = use_location().pathname;

    view! {
        <div class = "p-1 m-1">
            <div class="flex items-center">
                <div class = "flex flex wrap gap-2 p-2">
                    <svg class = "w-10 h-10" viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" role="img" class="iconify iconify--noto" preserveAspectRatio="xMidYMid meet" fill="currentColor"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier">
                        <path d="M100.08 16.44H75.44c-1.24 0-2.24 1-2.24 2.24v45.53c.31 10.84-.18 23.69-1.52 25.89c-.1.13-.2.28-.27.43c-1.6 3.33-6.9 6.78-12.99 6.78c-6.33 0-13.78-5.46-15.94-11.68a2.24 2.24 0 0 0-2.79-1.41l-20.47 6.41c-1.06.33-1.7 1.38-1.53 2.48c.01.07.03.14.04.2c2.32 17.58 18.94 29.38 41.46 29.38c22.53 0 39.16-11.81 41.48-29.4c.02-.1.04-.19.05-.29c1.56-15.02 1.58-28.71 1.58-28.85V18.68c.02-1.23-.98-2.24-2.22-2.24z" fill="currentColor"></path></g>
                    </svg>
                    <div class="hidden md:block">
                        <div class = "text-base font-bold">Jabra.</div>
                        <div class = "text-xs text-gray-500">Technology</div>
                    </div>
                </div>
            </div>

            <div class="divider divider-neutral mt-0 mb-0"></div>

            <MenuWithSubMenu
                is_selected_collapse
                sub_menu_open = location.get() == "/quote_builder" || location.get() == "/active_quotes"
                title = "QUOTES".to_string()
            >
                <Anchor
                    is_selected
                    location = location.get()
                    anchor_url = "/quote_builder".to_string()
                    title = "Quote Builder".to_string()
                />
                <Anchor
                    is_selected = is_selected
                    location = location.get()
                    anchor_url = "/active_quotes".to_string()
                    title = "Active Quotes".to_string()
                />
            </MenuWithSubMenu>

            <div class="divider divider-neutral mt-0 mb-0"></div>

            <MenuWithSubMenu
                is_selected_collapse
                sub_menu_open = location.get() == "/positions" || location.get() == "/trade_history"
                title = "TRADES".to_string()
            >
                <Anchor
                    is_selected
                    location = location.get()
                    anchor_url = "/positions".to_string()
                    title = "Positions".to_string()
                />
                <Anchor
                    is_selected = is_selected
                    location = location.get()
                    anchor_url = "/trade_history".to_string()
                    title = "Trade History".to_string()
                />
            </MenuWithSubMenu>
        </div>
    }
}

#[component]
pub fn Anchor<F>(is_selected: F, location: String, anchor_url: String, title: String) -> impl IntoView 
where F: Fn(bool) -> &'static str + 'static + Clone,
{
    let anchor_url_clone = anchor_url.clone();
    view! {
        <a class = {move || is_selected(location.clone() == {format!("{}", anchor_url.clone())})} class="hover:bg-base-100" href = {format!("{}", anchor_url_clone.clone())}>
            <span class="text-center">{format!("{}", title.clone())}</span>
        </a>
    }
}

#[component]
pub fn MenuWithSubMenu<T>(
    is_selected_collapse: T,
    sub_menu_open: bool,
    title: String,
    children: ChildrenFn,
) -> impl IntoView 
where 
T: Fn(bool) -> &'static str + 'static + Clone,
{
    view! {
        <details class="collapse collapse-arrow bg-base-200 hidden md:block" open={move || sub_menu_open}>
            <summary class="collapse-title text-md p-2 hover:bg-base-100">
            <div class="flex flex-row gap-2 items-center">
                <span>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
                        <path fill-rule="evenodd" d="M1 2.75A.75.75 0 011.75 2h16.5a.75.75 0 010 1.5H18v8.75A2.75 2.75 0 0115.25 15h-1.072l.798 3.06a.75.75 0 01-1.452.38L13.41 18H6.59l-.114.44a.75.75 0 01-1.452-.38L5.823 15H4.75A2.75 2.75 0 012 12.25V3.5h-.25A.75.75 0 011 2.75zM7.373 15l-.391 1.5h6.037l-.392-1.5H7.373zm7.49-8.931a.75.75 0 01-.175 1.046 19.326 19.326 0 00-3.398 3.098.75.75 0 01-1.097.04L8.5 8.561l-2.22 2.22A.75.75 0 115.22 9.72l2.75-2.75a.75.75 0 011.06 0l1.664 1.663a20.786 20.786 0 013.122-2.74.75.75 0 011.046.176z" clip-rule="evenodd" />
                    </svg>
                </span>
                <span>{format!("{}", title.clone())}</span>
            </div>
            </summary>
            <div class="collapse-content text-sm">
                {children()}
            </div>
        </details>

        // ------------------ MOBILE ------------------

        <div class = "flex flex-col gap-2 block md:hidden">
            <div class="dropdown dropdown-right">
                <div tabindex="0" role="button" class="hover:bg-base-100" class = {move || is_selected_collapse(sub_menu_open)}>
                    <span>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-6 h-6">
                        <path fill-rule="evenodd" d="M1 2.75A.75.75 0 011.75 2h16.5a.75.75 0 010 1.5H18v8.75A2.75 2.75 0 0115.25 15h-1.072l.798 3.06a.75.75 0 01-1.452.38L13.41 18H6.59l-.114.44a.75.75 0 01-1.452-.38L5.823 15H4.75A2.75 2.75 0 012 12.25V3.5h-.25A.75.75 0 011 2.75zM7.373 15l-.391 1.5h6.037l-.392-1.5H7.373zm7.49-8.931a.75.75 0 01-.175 1.046 19.326 19.326 0 00-3.398 3.098.75.75 0 01-1.097.04L8.5 8.561l-2.22 2.22A.75.75 0 115.22 9.72l2.75-2.75a.75.75 0 011.06 0l1.664 1.663a20.786 20.786 0 013.122-2.74.75.75 0 011.046.176z" clip-rule="evenodd" />
                        </svg>
                    </span>
                </div>
                <div class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">
                    {children()}
                </div>  
            </div>
        </div>
    }
}