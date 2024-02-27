use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn GeneratedQuotes() -> impl IntoView {
    view! {
        <div class = "flex-1 basis-auto rounded-md bg-base-300">
            <div class = "flex flex-col">
                <div class = "pb-7">
                    <table class = "table table-zebra table-xs">
                        <thead class = "text-base text-success font-extralight bg-base-300">
                            <tr class = "text-center border-b border-b-gray-800">
                                <th colspan = "11" class = "text-success">GENERATED QUOTES</th>
                            </tr>
                            <tr class="border-y border-y-gray-800">
                                <th>PAIR</th>
                                <th>AMOUNT</th>
                                <th>PRICE</th>
                                <th>QUOTE EXPIRY</th>
                                <th>COUNTERPARTY</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="hover">
                                <td>"BTC/USD"</td>
                                <td>"69"</td>
                                <td>"69420.69"</td>
                                <td>"Good Till Canceled"</td>
                                <td>"Herdbit Inc."</td>
                                <td>
                                    <button class = "btn btn-square btn-xs btn-warning">
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
                                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM6.75 9.25a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-6.5z" clip-rule="evenodd" />
                                        </svg>
                                    </button>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <div class = "flex flex-row-reverse items-center mb-2 border-t border-t-gray-800 p-4">
                    <div>
                        <button class = "btn btn-sm btn-info">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.28 7.22a.75.75 0 00-1.06 1.06L8.94 10l-1.72 1.72a.75.75 0 101.06 1.06L10 11.06l1.72 1.72a.75.75 0 101.06-1.06L11.06 10l1.72-1.72a.75.75 0 00-1.06-1.06L10 8.94 8.28 7.22z" clip-rule="evenodd" />
                            </svg>
                            CLEAR
                        </button>
                    </div>
                    <div>
                        <button class = "btn btn-sm btn-success mr-4">SUBMIT</button>
                    </div>
                </div>
            </div>
        </div>
    }
}