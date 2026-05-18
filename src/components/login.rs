use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="flex items-center justify-center w-screen h-screen bg-gradient-to-br from-teal-700 via-teal-600 to-teal-400">


            <div class="flex flex-col items-center gap-8 bg-white rounded-3xl shadow-2xl px-10 py-12 w-full max-w-sm mx-4">

                // title
                <div class="flex flex-col items-center gap-3">
                    <div class="text-center">
                        <h1 class="text-2xl font-bold text-gray-800">{"Yew Chat"}</h1>
                        <p class="text-sm text-gray-700 mt-1">{"Enter your username to start chatting"}</p>
                    </div>
                </div>

                // Input + button
                <div class="flex flex-col gap-3 w-full">
                    <div class="flex flex-col gap-1">
                        <label class="text-xs font-semibold text-teal-600 uppercase tracking-wider pl-1">
                            {"Username"}
                        </label>
                        <input
                            {oninput}
                            type="text"
                            placeholder="e.g. burhan123"
                            class="w-full px-4 py-3 rounded-xl border border-teal-200 bg-teal-50 text-gray-800 text-sm placeholder-teal-300 outline-none focus:ring-2 focus:ring-teal-400 focus:border-transparent transition-all"
                        />
                    </div>

                    <Link<Route> to={Route::Chat}>
                        <button
                            {onclick}
                            disabled={username.len() < 1}
                            class="w-full py-3 rounded-xl bg-teal-500 hover:bg-teal-600 disabled:bg-teal-200 disabled:cursor-not-allowed text-white font-semibold text-sm tracking-wide transition-all active:scale-95 shadow-md shadow-teal-100"
                        >
                            {"Go Chatting!"}
                        </button>
                    </Link<Route>>
                </div>

            </div>
        </div>
    }
}