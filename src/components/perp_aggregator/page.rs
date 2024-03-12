use leptos::*;

use crate::components::common::components::{common_attributes::ComponentSize, common_icons::Icon, input::InputNumber};

#[allow(non_snake_case)]
#[component]
pub fn PerpAggregator() -> impl IntoView {
    let size = create_rw_signal(0.0);

    view! {
        <div class="p-4">
            <Top />

            <span class="col-span-2 text-lg text-success font-semibold pl-1">Fill Calculator</span>
            <div class="grid grid-rows-2 grid-flow-col gap-4 rounded-lg bg-base-300 mt-2">
                <div class="p-4 rounded-lg bg-base-300 grid col-span-1">
                    <div class = "flex flex-col gap-4">
                        <label for="size" class="label">
                            <span class="text-lg text-success font-semibold">Size</span>
                        </label>
                        <input 
                            class="input input-md input-bordered w-full max-w-xs rounded hover:shadow-md"
                            type="number"
                            name="size"
                            min="1.0"
                            step="0.01"
                            value=size
                        />
                        <label for="side" class="label">
                            <span class="text-lg text-success font-semibold">Side</span>
                        </label>
                        <select class = "select select-md select-bordered text-success w-full max-w-xs rounded hover:shadow-sm hover:shadow-success bg-base-100 shadow-md" name="side">
                            <option value="Buy".to_string()>Buy</option>
                            <option value="Sell".to_string()>Sell</option>
                        </select>
                    </div>
                </div>

                <div class="p-9 rounded-lg shadow-lg grid row-span-2 col-span-2">
                    <table class="table outline outline-1 outline-success">
                        <thead>
                            <tr class="text-success font-semibold">
                                <th>Size</th>
                                <th>Ave. Fill Price</th>
                                <th>Venues</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>100</td>
                                <td>3500</td>
                                <td class="flex flex-row gap-2">
                                    <Icon title="BTC".to_string() size="w-5 h-5".to_string() />
                                    <Icon title="ETH".to_string() size="w-5 h-5".to_string() />
                                    <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                                </td>
                            </tr>
                            <tr>
                                <td>150</td>
                                <td>3500.3</td>
                                <td class="flex flex-row gap-2">
                                    <Icon title="ETH".to_string() size="w-5 h-5".to_string() />
                                    <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                                </td>
                            </tr>
                            <tr>
                                <td>200</td>
                                <td>3500.6</td>
                                <td class="flex flex-row gap-2">
                                    <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <div class="px-4 pt-6 rounded-lg bg-base-300 grid col-span-1 justify-center">
                    <div class="pt-6">
                        <div class="stat-title font-light">Average Fill Price</div>
                        <div class="stat-value bg-opacity-75"><span class = "text-success">4200.69</span></div>
                    </div>
                </div>

                <div class="p-4 rounded-lg shadow-lg bg-base-300 grid row-span-3">
                    <Routes />
                </div>
            </div>

            <div class="flex justify-around mt-4">
                <span class="text-lg text-success font-semibold">Bids</span>
                <span class="text-lg text-success font-semibold">Asks</span>
            </div>

            <div class="grid grid-cols-2 gap-4">
                <div class="overflow-x-auto">
                    <table class="table bg-green-400">
                        <thead>
                            <tr class="text-black font-semibold">
                                <th>Venues</th>
                                <th>Price</th>
                                <th>Size</th>
                                <th>Total</th>
                            </tr>
                        </thead>
                        <tbody class="text-black">
                            <tr>
                                <td class="flex flex-row gap-2">
                                    <Icon title="BTC".to_string() size="w-5 h-5".to_string() />
                                    <Icon title="ETH".to_string() size="w-5 h-5".to_string() />
                                    <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                                </td>
                                <td>3500</td>
                                <td>100</td>
                                <td>100</td>
                            </tr>
                            <tr>
                                <td class="flex flex-row gap-2">
                                    <Icon title="ETH".to_string() size="w-5 h-5".to_string() />
                                    <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                                </td>
                                <td>3400</td>
                                <td>100</td>
                                <td>100</td>
                            </tr>
                            <tr>
                                <td class="flex flex-row gap-2">
                                    <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                                </td>
                                <td>3300</td>
                                <td>100</td>
                                <td>100</td>
                            </tr>
                            <tr>
                                <td class="flex flex-row gap-2">
                                    <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                                </td>
                                <td>3300</td>
                                <td>100</td>
                                <td>100</td>
                            </tr>
                            <tr>
                                <td class="flex flex-row gap-2">
                                    <Icon title="ETH".to_string() size="w-5 h-5".to_string() />
                                    <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                                </td>
                                <td>3400</td>
                                <td>100</td>
                                <td>100</td>
                            </tr>
                            <tr>
                                <td class="flex flex-row gap-2">
                                    <Icon title="BTC".to_string() size="w-5 h-5".to_string() />
                                    <Icon title="ETH".to_string() size="w-5 h-5".to_string() />
                                    <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                                </td>
                                <td>3500</td>
                                <td>100</td>
                                <td>100</td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <div class="overflow-x-auto">
                    <table class="table bg-error">
                        <thead>
                            <tr class="text-black font-semibold">
                                <th>Venues</th>
                                <th>Price</th>
                                <th>Size</th>
                                <th>Total</th>
                            </tr>
                        </thead>
                        <tbody class="text-black">
                            <tr>
                                <td class="flex flex-row gap-2">
                                    <Icon title="BTC".to_string() size="w-5 h-5".to_string() />
                                    <Icon title="ETH".to_string() size="w-5 h-5".to_string() />
                                    <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                                </td>
                                <td>3500</td>
                                <td>100</td>
                                <td>100</td>
                            </tr>
                            <tr>
                                <td class="flex flex-row gap-2">
                                    <Icon title="ETH".to_string() size="w-5 h-5".to_string() />
                                    <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                                </td>
                                <td>3400</td>
                                <td>100</td>
                                <td>100</td>
                            </tr>
                            <tr>
                                <td class="flex flex-row gap-2">
                                    <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                                </td>
                                <td>3300</td>
                                <td>100</td>
                                <td>100</td>
                            </tr>
                            <tr>
                                <td class="flex flex-row gap-2">
                                    <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                                </td>
                                <td>3300</td>
                                <td>100</td>
                                <td>100</td>
                            </tr>
                            <tr>
                                <td class="flex flex-row gap-2">
                                    <Icon title="ETH".to_string() size="w-5 h-5".to_string() />
                                    <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                                </td>
                                <td>3400</td>
                                <td>100</td>
                                <td>100</td>
                            </tr>
                            <tr>
                                <td class="flex flex-row gap-2">
                                    <Icon title="BTC".to_string() size="w-5 h-5".to_string() />
                                    <Icon title="ETH".to_string() size="w-5 h-5".to_string() />
                                    <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                                </td>
                                <td>3500</td>
                                <td>100</td>
                                <td>100</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>


            // <div class="pt-9 px-4 outline outline-1 rounded">
                // <FillCalculator />
            // </div>

        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Top() -> impl IntoView {
    view! {
        <div class="flex justify-between mb-9">
            <div>
                <select class = "select select-md select-bordered text-success w-full max-w-xs rounded hover:shadow-sm hover:shadow-success bg-base-100 shadow-md" name="currency_pair">
                    <option disabled selected>Select Pair</option>
                    <option value="BTC/USD".to_string()>BTC/USD</option>
                    <option value="BTC/USDC".to_string()>BTC/USDC</option>
                    <option value="ETH/USD".to_string()>ETH/USD</option>
                </select>
            </div>
            <div>
                <div class="flex flex-row gap-4 items-end">
                    <span class="font-light">24hr Volume</span>
                    <span class="font-semibold">$150B</span>
                    <span class="text-success">+50%</span>
                </div>
                <div class="flex flex-row gap-4 items-end">
                    <span class="font-light">Open Interest</span>
                    <span class="font-semibold">$60B</span>
                    <span class="text-success">+4%</span>
                </div>
            </div>
        </div>
    }
}

// #[allow(non_snake_case)]
// #[component]
// pub fn FillCalculator() -> impl IntoView {
//     let size = create_rw_signal(0.0);
//     view! {
//         <div class = "text-xl font-semibold text-success">
//             <span>Fill Calculator</span>
//         </div>
//         <div class="p-4">
//             <div class="flex flex-col gap-4">
//                 <div class="flex flex-row gap-4">
//                     <div class = "flex flex-col gap-4">
//                         <InputNumber
//                             name="size".to_string()
//                             label="Size".to_string()
//                             value=size
//                             min=1.0
//                             step=0.01
//                             size=ComponentSize::Small
//                         />
//                         <select class = "select select-sm text-success w-full max-w-xs rounded hover:shadow-sm hover:shadow-success bg-base-100 shadow-md" name="side">
//                             <option disabled selected>Select Side</option>
//                             <option value="Buy".to_string()>Buy</option>
//                             <option value="Sell".to_string()>Sell</option>
//                         </select>
//                     </div>

//                     <div>
//                         <div class="stats shadow mr-2">
//                             <div class="stat">
//                                 <div class="stat-title font-light">Average Fill Price</div>
//                                 <div class="stat-value bg-opacity-75"><span class = "text-success">4200.69</span></div>
//                             </div>
//                         </div>
//                     </div>

//                     <div class="flex flex-col gap-4 justify-end">
//                         <Routes />
//                     </div>
//                 </div>
                
//                 <div>
//                     <span class = "text-md font-medium text-success">Bump Table</span>
//                     <div class = "skeleton flex w-full h-32 border border-gray-800 rounded-md bg-base-300 items-center">
//                         <span class="loading loading-bars loading-sm text-success"></span>
//                     </div>
//                 </div>
//             </div>

//         </div>
//     }
// }

#[allow(non_snake_case)]
#[component]
pub fn Routes() -> impl IntoView {
    view! {
        // <span class = "text-md font-medium text-success p-4">Routes</span>
        <div class="flex flex-row gap-4 min-h-full">
            <div class="p-4 rounded flex justify-center items-center outline outline-1 mr-4">
                <Icon title="USDC".to_string() size="w-7 h-7".to_string() />
            </div>
            <div class="flex flex-col gap-11 justify-center">
                <div class="flex flex-row gap-9 items-center">
                    <div class="indicator p-1">
                        <span class="indicator-item indicator-start badge badge-primary p-1">20%</span>
                        <div class="flex flex-row gap-1 p-2 rounded outline outline-1 items-center">
                            <Icon title="USD".to_string() size="w-5 h-5".to_string() />
                            <span>USD</span>
                        </div>
                    </div>
                    <span class="p-2">
                        <Icon title="ARROW-RIGHT".to_string() size="w-5 h-5 animate-bounce".to_string() />
                    </span>
                    <div class="flex flex-row gap-1 p-2 rounded outline outline-1 items-center">
                        <Icon title="BINANCE".to_string() size="w-5 h-5".to_string() />
                        <span>Binance</span>
                    </div>
                    <span class="p-2">
                        <Icon title="ARROW-RIGHT".to_string() size="w-5 h-5 animate-bounce".to_string() />
                    </span>
                    <span class="p-2 rounded outline outline-1">ERC721 <span class="text-success text-xs">BTC-14MAR23-62034-C</span></span>
                </div>

                <div class="flex flex-row gap-9 items-center">
                    <div class="indicator p-1">
                        <span class="indicator-item indicator-start badge badge-primary p-1">30%</span>
                        <div class="flex flex-row gap-1 p-2 rounded outline outline-1 items-center">
                            <Icon title="USDC".to_string() size="w-5 h-5".to_string() />
                            <span>USDC</span>
                        </div>
                    </div>
                    <span class="p-2">
                        <Icon title="ARROW-RIGHT".to_string() size="w-5 h-5 animate-bounce".to_string() />
                    </span>
                    <div class="flex flex-row gap-1 p-2 rounded outline outline-1 items-center">
                        <Icon title="BINANCE".to_string() size="w-5 h-5".to_string() />
                        <span>Binance</span>
                    </div>
                    <span class="p-2">
                        <Icon title="ARROW-RIGHT".to_string() size="w-5 h-5 animate-bounce".to_string() />
                    </span>
                    <span class="p-2 rounded outline outline-1">ERC721 <span class="text-success text-xs">BTC-14MAR23-62034-C</span></span>
                </div>

                <div class="flex flex-row gap-9 items-center">
                    <div class="indicator p-1">
                        <span class="indicator-item indicator-start badge badge-primary p-1">10%</span>
                        <div class="flex flex-row gap-1 p-2 rounded outline outline-1 items-center">
                            <Icon title="BTC".to_string() size="w-5 h-5".to_string() />
                            <span>BTC</span>
                        </div>
                    </div>
                    <span class="p-2">
                        <Icon title="ARROW-RIGHT".to_string() size="w-5 h-5 animate-bounce".to_string() />
                    </span>
                    <div class="flex flex-row gap-1 p-2 rounded outline outline-1 items-center">
                        <Icon title="BINANCE".to_string() size="w-5 h-5".to_string() />
                        <span>Binance</span>
                    </div>
                    <span class="p-2">
                        <Icon title="ARROW-RIGHT".to_string() size="w-5 h-5 animate-bounce".to_string() />
                    </span>
                    <span class="p-2 rounded outline outline-1">ERC721 <span class="text-success text-xs">BTC-14MAR23-62034-C</span></span>
                </div>

                <div class="flex flex-row gap-9 items-center">
                    <div class="indicator p-1">
                        <span class="indicator-item indicator-start badge badge-primary p-1">40%</span>
                        <div class="flex flex-row gap-1 p-2 rounded outline outline-1 items-center">
                            <Icon title="ETH".to_string() size="w-5 h-5".to_string() />
                            <span>ETH</span>
                        </div>
                    </div>
                    <span class="p-2">
                        <Icon title="ARROW-RIGHT".to_string() size="w-5 h-5 animate-bounce".to_string() />
                    </span>
                    <div class="flex flex-row gap-1 p-2 rounded outline outline-1 items-center">
                        <Icon title="BINANCE".to_string() size="w-5 h-5".to_string() />
                        <span>Binance</span>
                    </div>
                    <span class="p-2">
                        <Icon title="ARROW-RIGHT".to_string() size="w-5 h-5 animate-bounce".to_string() />
                    </span>
                    <span class="p-2 rounded outline outline-1 animate-pulse">ERC721 <span class="text-success text-xs">BTC-14MAR23-62034-C</span></span>
                </div>
            </div>
        </div>
    }
}
