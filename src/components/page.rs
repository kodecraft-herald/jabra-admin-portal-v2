use leptos::*;
use leptos_router::use_location;

use crate::components::{
    component_testing::ComponentTesting,
    content::{Content1, Content2},
    login::Login,
    sidebar::Sidebar,
};

#[component]
pub fn Page() -> impl IntoView {
    let location = use_location().pathname;

    view! {
        <div class = "main-content gap-2 flex">
            <svg class="svg-bg translate-y-1/4 md:translate-y-0" viewBox="0 0 1728 451" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M1728 1L1712.66 31.3737L1699.12 48.0303L1691.9 31.3737L1682.88 51.9495L1675.66 48.0303L1650.39 99.9596L1627.83 77.4242L1620.61 93.101L1605.27 77.4242L1586.32 126.414L1572.78 110.737L1556.54 142.091L1531.27 163.646L1513.22 182.263L1491.56 210.677L1450.95 169.525L1412.15 191.081L1397.71 234.192L1374.25 255.747L1359.81 225.374L1343.57 218.515L1336.35 210.677L1318.3 225.374L1304.76 245.949L1293.93 218.515L1276.79 191.081L1270.47 169.525L1250.62 163.646L1237.08 138.172L1222.64 132.293L1210.01 99.9596L1199.18 77.4242L1170.3 70.5657L1132.4 48.0303L1108.03 85.2626L1089.08 126.414L1070.13 138.172L1051.18 159.727L1030.42 191.081L1014.18 186.182L1002.45 163.646L991.619 151.889L981.692 126.414L960.936 146.99L935.668 195.98L914.912 218.515L896.864 234.192L880.62 218.515L871.596 182.263L850.84 146.99L819.255 245.949L786.767 299.838L766.012 344.909L750.67 336.091L726.305 356.667L701.037 362.545L686.598 377.242L645.989 366.465L610.794 389L592.851 344.909L568.769 356.667L530.049 362.545L499.829 336.091L487.08 362.545L463.942 377.242L421.917 424L378.948 450L324.173 412L231.152 424L190.543 377.242L123.02 424L73.9117 398.5L32.3588 356.667L10.1658 344.909L-4 362.545" stroke="currentColor" class="animate-chart-line"></path><defs><linearGradient id="paint0_linear_22_438" x1="1769.55" y1="237.529" x2="-84" y2="511.5" gradientUnits="userSpaceOnUse"><stop stop-color="white"></stop><stop offset="0.288039" stop-color="white" stop-opacity="0.3"></stop><stop offset="1" stop-color="white" stop-opacity="0"></stop></linearGradient></defs></svg>
            // <div class = "flex-none basis-1/12 md:basis-1/6 rounded-xl min-h-full bg-gradient-to-bl from-deep to-light">
            <div class = "flex-none basis-1/12 md:basis-1/6 rounded-xl min-h-full bg-base-300">
                <Sidebar />
            </div>
            // <div class = "flex-1 rounded-xl min-h-full bg-gradient-to-br from-deep to-light">
            <div class = "flex-1 rounded-xl min-h-full bg-base-300">
                {
                    move || {
                        match location.get().as_str() {
                            "/" => view!{<Content1 />},
                            "/login" => view!{<Login />},
                            "/quote_builder" => view!{<Content1 />},
                            "/active_quotes" => view!{<Content1 />},
                            "/positions" => view!{<Content2 />},
                            "/trade_history" => view!{<Content2 />},
                            "/components" => view!{<ComponentTesting />},

                            _ => view!{<div></div>}.into_view(),
                        }
                    }
                }
            </div>
        </div>
    }
}
