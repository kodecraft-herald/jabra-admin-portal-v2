use leptos::*;

use crate::components::common::components::common_icons::Icon;

#[allow(non_snake_case)]
#[component]
pub fn PerpAggregator() -> impl IntoView {
    view! {
        <div class="p-6">

            <Top />

            <span class="text-lg text-success font-normal pl-1">Fill Calculator</span>
            <div class="grid grid-rows-2 grid-flow-col gap-4 rounded-lg bg-base-300 mt-2 shadow-lg border border-1 border-success">

                <FillCalculatorInputs />

                <div class="rounded-lg grid row-span-2 col-span-2">

                    <BumpTable />

                </div>

                <div class="px-4 pt-4 rounded-lg grid col-span-1 justify-center">
                    <div class="pt-4">
                        <div class="stat-title font-light">Average Fill Price</div>
                        <div class="stat-value bg-opacity-75"><span class = "text-success">4200.69</span></div>
                    </div>
                </div>

                <div class="p-4 rounded-lg grid row-span-3">

                    <Routes />

                </div>
            </div>

            <BidAsks />
            
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn FillCalculatorInputs() -> impl IntoView {
    let size = create_rw_signal(0.0);
    view! {
        <div class="p-4 rounded-lg grid col-span-1">
            <div class = "flex flex-col gap-4">
                <div>
                    <label for="size" class="label">
                        <span class="text-md text-success font-normal">Size</span>
                    </label>
                    <input 
                        class="input input-sm text-xs input-bordered w-full max-w-xs rounded hover:shadow-md"
                        type="number"
                        name="size"
                        min="1.0"
                        step="0.01"
                        value=size
                    />
                </div>
                <div>
                    <label for="side" class="label">
                        <span class="text-md text-success font-normal">Side</span>
                    </label>
                    <select class = "select select-sm text-xs select-bordered text-success w-full max-w-xs rounded hover:shadow-sm hover:shadow-success bg-base-100 shadow-md" name="side">
                        <option value="Buy".to_string()>Buy</option>
                        <option value="Sell".to_string()>Sell</option>
                    </select>
                </div>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn BumpTable() -> impl IntoView {
    view! {
        <div class="p-9">
            <p class="text-lg text-success font-normal pl-1 pb-1">Bump Table</p>
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
    }
}

#[allow(non_snake_case)]
#[component]
pub fn BidAsks() -> impl IntoView {
    view! {
        <div class="flex justify-around mt-4 pb-2">
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
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Top() -> impl IntoView {
    view! {
        <div class="flex justify-between mb-8">
            <div class="flex flex-col gap-4">
                <span class="text-lg text-success font-semibold">[Perp Aggregator]</span>
                <select class = "select select-sm text-xs select-bordered text-success w-full max-w-xs rounded hover:shadow-sm hover:shadow-success bg-base-100 shadow-md" name="currency_pair">
                    <option disabled selected>Select Pair</option>
                    <option value="BTC/USD".to_string()>BTC/USD</option>
                    <option value="BTC/USDC".to_string()>BTC/USDC</option>
                    <option value="ETH/USD".to_string()>ETH/USD</option>
                </select>
            </div>

            <div class="flex flex-col items-end pr-1">
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

#[allow(non_snake_case)]
#[component]
pub fn Routes() -> impl IntoView {
    view! {
        <div>
            <p class = "text-lg font-normal text-success py-2">Routes</p>
            <div class="flex flex-row gap-4 min-h-90">
                <div class="p-4 rounded flex justify-center items-center outline outline-1 mr-4">
                    <Icon title="USDC".to_string() size="w-7 h-7".to_string() />
                </div>
                <div class="flex flex-col justify-around py-2">
                    <div class="flex flex-row gap-4 lg:gap-9 items-center">
                        <div class="indicator connector-end">
                            <span class="indicator-item indicator-start badge badge-primary p-1">20%</span>
                            <div class="flex flex-row gap-1 p-2 rounded outline outline-1 items-center">
                                <Icon title="USD".to_string() size="w-5 h-5".to_string() />
                                <span>USD</span>
                            </div>
                        </div>
                        // <span class="p-2">
                        //     <Icon title="ARROW-RIGHT".to_string() size="w-5 h-5 animate-bounce".to_string() />
                        // </span>
                        <div class="connector-end">
                            <div class="flex flex-row gap-1 p-2 rounded outline outline-1 items-center">
                                <Icon title="BINANCE".to_string() size="w-5 h-5".to_string() />
                                <span>Binance</span>
                            </div>
                        </div>
                        
                        // <span class="p-2">
                        //     <Icon title="ARROW-RIGHT".to_string() size="w-5 h-5 animate-bounce delay-1500".to_string() />
                        // </span>
                        <p class="p-2 rounded outline outline-1 text-xs">ERC721 <span class="text-success text-xs">BTC-14MAR23-62034-C</span></p>
                    </div>

                    <div class="flex flex-row gap-4 lg:gap-9 items-center">
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
                            <Icon title="ARROW-RIGHT".to_string() size="w-5 h-5 animate-bounce delay-1500".to_string() />
                        </span>
                        <p class="p-2 rounded outline outline-1 text-xs">ERC721 <span class="text-success text-xs">BTC-14MAR23-62034-C</span></p>
                    </div>

                    <div class="flex flex-row gap-4 lg:gap-9 items-center">
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
                            <Icon title="ARROW-RIGHT".to_string() size="w-5 h-5 animate-bounce delay-1500".to_string() />
                        </span>
                        <p class="p-2 rounded outline outline-1 text-xs">ERC721 <span class="text-success text-xs">BTC-14MAR23-62034-C</span></p>
                    </div>

                    <div class="flex flex-row gap-4 lg:gap-9 items-center">
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
                            <Icon title="ARROW-RIGHT".to_string() size="w-5 h-5 animate-bounce delay-1500".to_string() />
                        </span>
                        <p class="p-2 rounded outline outline-1 animate-pulse text-xs">ERC721 <span class="text-success text-xs">BTC-14MAR23-62034-C</span></p>
                    </div>
                </div>
            </div>
        </div>
    }
}
