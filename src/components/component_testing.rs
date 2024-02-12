use leptos::*;
use crate::components::common::components::input::{InputNumber, InputPassword, InputText};


#[island]
pub fn ComponentTesting() -> impl IntoView {
    let selected_component = create_rw_signal("inputtext".to_string());
    let value = create_rw_signal(0.0);

    let render_component = move || {
        match selected_component.get().as_str() {
            "inputtext" => view! {
                <InputText
                    name="userid".to_string()
                    label="Email".to_string()
                    custom_class="input input-bordered w-full rounded hover:shadow-md".to_string()
                    icon="Email".to_string()
                />
            },
            "inputpassword" => view! {
                <InputPassword
                    name="password".to_string()
                    label="Password".to_string()
                    custom_class="input input-bordered w-full rounded hover:shadow-md".to_string()
                    icon="Password".to_string()
                />
            },

            "inputnumber" => view! {
                <InputNumber
                    name="inputnumber".to_string()
                    label="Input Number".to_string()
                    value=value
                    min=1.0
                    step=0.01
                />
            },
            _ => view! {
                <h1>Select a component to render.</h1>
            }.into_view()
        }
    };

    create_effect(move |_| {
        log::info!("selected: {:?}", selected_component.get());
    });

    view! {
        <div class="p-4">
            <div class="flex justify-between items-center gap-4">
                <div class="card w-full shadow-lg bg-base-100">
                    <div class="card-body">
                        <label class="label">
                            <span class="label-text">Select a Component</span>
                        </label>
                        <select class="select select-success w-full max-w-xs"
                            on:change = move |e| {
                                let val = event_target_value(&e);
                                selected_component.set(val);
                            }
                        >
                            <option value="inputtext">Input Text</option>
                            <option value="inputpassword">Input Password</option>
                            <option value="inputnumber">Input Number</option>
                        </select>
                    </div>
                </div>
                <div class="card w-full shadow-lg bg-base-100">
                    <div class="card-body">
                        {
                            move || {
                                render_component()
                            }
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}