use leptos::*;

#[component]
pub fn Login() -> impl IntoView {

    view! {
        <div class="h-full lg:grid lg:grid-cols-3">
            <div class = "h-full flex items-center justify-center px-4">
                <div class="card flex-shrink-0 w-full max-w-sm shadow-lg bg-light">
                    <form>
                        <div class="card-body">
                            <div class="form-control">
                                <label for = "userid" class="label">
                                <span class="label-text">Email</span>
                                </label>
                                <input type="text" id = "userid" name = "userid" class="input input-sm bg-white rounded hover:shadow-md text-black border-gray-800 shadow-md" autocomplete required 
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                <span for = "password" class="label-text">Password</span>
                                </label>
                                <input type="password" id = "password" name = "password" class="input input-sm bg-white rounded hover:shadow-md text-black border-gray-800 shadow-md" autocomplete required 
                                />
                                <label class="label">
                                <a href="#" class="label-text-alt link link-hover">Forgot password?</a>
                                </label>
                            </div>
                            <div class="form-control mt-6">
                                <button type = "submit" class="btn bg-gradient-to-bl from-deep to-light shadow">LOGIN</button>
                            </div>
                        </div>
                    </form>
                </div>
            </div>
            <div class = "h-full lg:flex items-center hidden lg:block lg:col-span-2">
                <div class = "flex flex-col">
                    <div class="flex items-center">
                        <h1 class="text-6xl text-white">
                        "Bespoke Structured Products"
                        <br/>
                        "for your"
                        <span class="font-bold">" digital assets"</span>
                        </h1>
                    </div>
                    <p class="mt-4">A tailored solution for your investment thesis.</p>
                </div>
            </div>
        </div>
    }
}