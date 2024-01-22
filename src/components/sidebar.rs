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
            <Quotes
                is_selected
                location
                is_selected_collapse
            />

            <div class="divider divider-neutral mt-0 mb-0"></div>
            <Trades
                is_selected
                location
                is_selected_collapse
            />

        </div>
    }
}

#[component]
pub fn Trades<F, T>(is_selected: F, location: Memo<String>, is_selected_collapse: T) -> impl IntoView 
where 
F: Fn(bool) -> &'static str + 'static + Clone,
T: Fn(bool) -> &'static str + 'static + Clone,

{

    let is_selected_clone = is_selected.clone();
    let is_selected_collapse_clone = is_selected_collapse.clone();
    let is_selected_collapse_clone_2 = is_selected_collapse.clone();

    view! {
        <details class="collapse collapse-arrow bg-base-200 hidden md:block" open={move || (location.get() == "/positions" || location.get() == "/trade_history")}>
            <summary class="collapse-title text-md p-2 hover:bg-base-100">
            <div class="flex flex-row gap-2 items-center">
                <span>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
                        <path fill-rule="evenodd" d="M1 2.75A.75.75 0 011.75 2h16.5a.75.75 0 010 1.5H18v8.75A2.75 2.75 0 0115.25 15h-1.072l.798 3.06a.75.75 0 01-1.452.38L13.41 18H6.59l-.114.44a.75.75 0 01-1.452-.38L5.823 15H4.75A2.75 2.75 0 012 12.25V3.5h-.25A.75.75 0 011 2.75zM7.373 15l-.391 1.5h6.037l-.392-1.5H7.373zm7.49-8.931a.75.75 0 01-.175 1.046 19.326 19.326 0 00-3.398 3.098.75.75 0 01-1.097.04L8.5 8.561l-2.22 2.22A.75.75 0 115.22 9.72l2.75-2.75a.75.75 0 011.06 0l1.664 1.663a20.786 20.786 0 013.122-2.74.75.75 0 011.046.176z" clip-rule="evenodd" />
                    </svg>
                </span>
                <span>TRADES</span>
            </div>
            </summary>
            <div class="collapse-content text-sm">
                <a class = {move || is_selected(location.get() == "/positions")} class="hover:bg-base-100" href = "/positions">
                    <span class="text-center">Positions</span>
                </a>
                <a class = {move || is_selected_clone(location.get() == "/trade_history")} class="hover:bg-base-100" href = "/trade_history">
                    <span class="text-center">Trade History</span>
                </a>
            </div>
        </details>
        

        <div class = "flex flex-col gap-2 block md:hidden">
            <div class="dropdown dropdown-right">
                <div tabindex="0" role="button" class="hover:bg-base-100" class = {move || is_selected_collapse_clone_2(location.get() == "/positions" || location.get() == "/trade_history")}>
                    <span>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-6 h-6">
                        <path fill-rule="evenodd" d="M1 2.75A.75.75 0 011.75 2h16.5a.75.75 0 010 1.5H18v8.75A2.75 2.75 0 0115.25 15h-1.072l.798 3.06a.75.75 0 01-1.452.38L13.41 18H6.59l-.114.44a.75.75 0 01-1.452-.38L5.823 15H4.75A2.75 2.75 0 012 12.25V3.5h-.25A.75.75 0 011 2.75zM7.373 15l-.391 1.5h6.037l-.392-1.5H7.373zm7.49-8.931a.75.75 0 01-.175 1.046 19.326 19.326 0 00-3.398 3.098.75.75 0 01-1.097.04L8.5 8.561l-2.22 2.22A.75.75 0 115.22 9.72l2.75-2.75a.75.75 0 011.06 0l1.664 1.663a20.786 20.786 0 013.122-2.74.75.75 0 011.046.176z" clip-rule="evenodd" />
                        </svg>
                    </span>
                </div>
                <div class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">
                    <a class = {move || is_selected_collapse(location.get() == "/positions")} class="hover:bg-base-200" href = "/positions">
                        <span class="text-center">Positions</span>
                    </a>
                    <a class = {move || is_selected_collapse_clone(location.get() == "/trade_history")} class="hover:bg-base-200" href = "/trade_history">
                        <span class="text-center">Trade History</span>
                    </a>
                </div>  
            </div>
        </div>
    }
}

#[component]
pub fn Quotes<F,T>(is_selected: F, location: Memo<String>, is_selected_collapse: T) -> impl IntoView 
where 
F: Fn(bool) -> &'static str + 'static + Clone,
T: Fn(bool) -> &'static str + 'static + Clone,
{

    let is_selected_clone = is_selected.clone();
    let is_selected_collapse_clone = is_selected_collapse.clone();
    let is_selected_collapse_clone_2 = is_selected_collapse.clone();

    view! {
        <details class="collapse collapse-arrow bg-base-200 hidden md:block" open={move || (location.get() == "/quote_builder" || location.get() == "/active_quotes")}>
            <summary class="collapse-title text-md p-2">
            <div class="flex flex-row gap-2 items-center hover:bg-base-100">
                <span>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
                    <path fill-rule="evenodd" d="M12.577 4.878a.75.75 0 01.919-.53l4.78 1.281a.75.75 0 01.531.919l-1.281 4.78a.75.75 0 01-1.449-.387l.81-3.022a19.407 19.407 0 00-5.594 5.203.75.75 0 01-1.139.093L7 10.06l-4.72 4.72a.75.75 0 01-1.06-1.061l5.25-5.25a.75.75 0 011.06 0l3.074 3.073a20.923 20.923 0 015.545-4.931l-3.042-.815a.75.75 0 01-.53-.919z" clip-rule="evenodd" />
                    </svg>
                </span>
                <span>QUOTES</span>
            </div>
            </summary>
            <div class="collapse-content text-sm">
                <a class = {move || is_selected(location.get() == "/quote_builder")} class="hover:bg-base-100" href = "/quote_builder">
                    <span class="text-center">Quote Builder</span>
                </a>
                <a class = {move || is_selected_clone(location.get() == "/active_quotes")} class="hover:bg-base-100" href = "/active_quotes">
                    <span class="text-center">Active Quotes</span>
                </a>
            </div>
        </details>

        <div class = "flex flex-col gap-2 block md:hidden">
            <div class="dropdown dropdown-right">
                <div tabindex="0" role="button" class="hover:bg-base-100" class = {move || is_selected_collapse_clone_2(location.get() == "/quote_builder" || location.get() == "/active_quotes")}>
                    <span>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-6 h-6">
                        <path fill-rule="evenodd" d="M12.577 4.878a.75.75 0 01.919-.53l4.78 1.281a.75.75 0 01.531.919l-1.281 4.78a.75.75 0 01-1.449-.387l.81-3.022a19.407 19.407 0 00-5.594 5.203.75.75 0 01-1.139.093L7 10.06l-4.72 4.72a.75.75 0 01-1.06-1.061l5.25-5.25a.75.75 0 011.06 0l3.074 3.073a20.923 20.923 0 015.545-4.931l-3.042-.815a.75.75 0 01-.53-.919z" clip-rule="evenodd" />
                        </svg>
                    </span>
                </div>
                <div class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">
                    <a class = {move || is_selected_collapse(location.get() == "/quote_builder")} class="hover:bg-base-200" href = "/quote_builder">
                        <span class="text-center">Quote Builder</span>
                    </a>
                    <a class = {move || is_selected_collapse_clone(location.get() == "/active_quotes")} class="hover:bg-base-200" href = "/active_quotes">
                        <span class="text-center">Active Quotes</span>
                    </a>
                </div>  
            </div>
        </div>
    }
}
