use crate::components::common::{
    components::{
        common_attributes::ComponentType,
        input::{InputDatePicker, InputNumber, InputPassword, InputText},
        range_selector::RangeSelector,
    },
    functions::helpers::extract_date_as_string,
};
use chrono::Utc;
use h_modals::{
    attributes::enums::{ComponentStatus, Position},
    confirm_modal::components::ConfirmModal,
    status_modal::components::StatusModal,
    status_modal_fn::components::StatusModalWithFunction,
};
use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn ComponentTesting() -> impl IntoView {
    let selected_component = create_rw_signal("inputtext".to_string());
    let value = create_rw_signal(0.0);
    let value_str = create_rw_signal("".to_string());
    let range_signal = create_rw_signal(0.0);
    let date_now = create_rw_signal(Utc::now());
    let min_date = move || extract_date_as_string(date_now.get());

    let render_component = move || match selected_component.get().as_str() {
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

        "inputdatepicker" => view! {
            <InputDatePicker
                name="inputdatepicker".to_string()
                label="Date Picker".to_string()
                date=date_now
                value=value_str
                min=min_date()
            />
        },

        "rangeselector" => view! {
            <RangeSelector
                name="rangeselector".to_string()
                label="Range Selector".to_string()
                step=2.0
                min=1.0
                max=20.0
                value_signal=range_signal
                component_type=ComponentType::Success
            />
        },

        "dialogbox" => view! {
            <DialogBoxComponent/>
        }
        .into_view(),

        "confirmmodal" => view! {
            <ConfirmModalComponent/>
        }
        .into_view(),
        _ => view! {
            <h1>Select a component to render.</h1>
        }
        .into_view(),
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
                            <option value="inputdatepicker">Date Picker</option>
                            <option value="rangeselector">Range Selector</option>
                            <option value="dialogbox">Dialog Box</option>
                            <option value="confirmmodal">Confirm Modal</option>
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

#[allow(non_snake_case)]
#[component]
pub fn DialogBoxComponent() -> impl IntoView {
    let success_modal = create_rw_signal(false);
    let error_modal = create_rw_signal(false);
    let info_modal = create_rw_signal(false);
    let warning_modal = create_rw_signal(false);
    let neutral_modal = create_rw_signal(false);

    let on_click_fn = move || {
        neutral_modal.set(false);
        log::info!("Do something!");
    };

    view! {
        <div class="grid grid-cols-3 gap-4">
            <button class = "btn btn-sm btn-success" on:click = move |_| success_modal.set(true)>Success Modal</button>
            <button class = "btn btn-sm btn-error" on:click = move |_| error_modal.set(true)>Error Modal</button>
            <button class = "btn btn-sm btn-warning" on:click = move |_| warning_modal.set(true)>Warning Modal</button>
            <button class = "btn btn-sm btn-info" on:click = move |_| info_modal.set(true)>Info Modal</button>
            <button class = "btn btn-sm btn-neutral" on:click = move |_| neutral_modal.set(true)>Neutral Modal</button>
        </div>
        <StatusModal
            signal=success_modal
            title="SUCCESS!".to_string()
            description="Trade quote approval is successful".to_string()
            status=ComponentStatus::Success
            position=Position::TopLeft
        />
        <StatusModal
            signal=error_modal
            title="ERROR!".to_string()
            description="This is an error description".to_string()
            status=ComponentStatus::Error
            position=Position::TopMiddle
        />
        <StatusModal
            signal=warning_modal
            title="WARNING!".to_string()
            description="This is a warning description.".to_string()
            status=ComponentStatus::Warning
            position=Position::TopRight
        />
        <StatusModal
            signal=info_modal
            title="INFO!".to_string()
            description="This is an info description".to_string()
            status=ComponentStatus::Info
            position=Position::Middle
            text_color=ComponentStatus::Info
            button_status=ComponentStatus::Info
        />
        <StatusModalWithFunction
            signal=neutral_modal
            title="NEUTRAL!".to_string()
            description="This is a neutral description".to_string()
            status=ComponentStatus::Neutral
            position=Position::BottomMiddle
            text_color=ComponentStatus::Neutral
            function=on_click_fn
        />
    }
}

#[allow(non_snake_case)]
#[component]
pub fn ConfirmModalComponent() -> impl IntoView {
    let signal = create_rw_signal(false);
    let action = create_action(move |_input: &()| async move {
        let result = server_function().await;
        match result {
            Ok(result) => {
                log::info!("Action Successful!");
                signal.set(false);
                return result;
            }
            Err(err) => {
                log::error!("Server Function Error: {:?}", err);
                signal.set(false);
                return format!("Server Function Error: {:?}", err);
            }
        }
    });
    let pending_signal = action.pending();

    let confirm_modal_fn = move || {
        log::info!("Confirm Modal Function");
        action.dispatch(());
    };

    view! {
        <button class = "btn btn-sm btn-success" on:click = move |_| signal.set(true)>Confirm Modal</button>
        <ConfirmModal
            signal = signal
            title = "APPROVE?".to_string()
            description = "Are you sure you want to approve?".to_string()
            function = confirm_modal_fn
            pending_signal = pending_signal
        />
    }
}

pub async fn server_function() -> Result<String, ServerFnError> {
    use gloo_timers::future::sleep;
    use std::time::Duration;

    log::info!("Server Function!");

    sleep(Duration::from_secs(1)).await;

    return Ok(String::from("This came from a server function."));
}
