use leptos::*;

use super::common_attributes::{ComponentSize, ComponentType};

#[allow(non_snake_case)]
#[component]
pub fn RangeSelector(
    name: String,
    label: String,
    step: f64,
    min: f64,
    max: f64,
    #[prop(optional)] size: Option<ComponentSize>,
    #[prop(optional)] component_type: Option<ComponentType>,
    value_signal: RwSignal<f64>, // For setting the value inside on:change
) -> impl IntoView {
    let type_class = match component_type {
        None => "",
        Some(ComponentType::Info) => "info",
        Some(ComponentType::Success) => "success",
        Some(ComponentType::Neutral) => "neutral",
        Some(ComponentType::Warning) => "warning",
        Some(ComponentType::Error) => "error",
    };

    let text_class = match size {
        None => "",
        Some(ComponentSize::ExtraSmall) => "text-xs",
        Some(ComponentSize::Small) => "text-sm",
        Some(ComponentSize::Base) => "text-md",
        Some(ComponentSize::Large) => "text-lg",
    };

    let input_class = match size {
        None => format!("range range-{}", type_class),
        Some(ComponentSize::ExtraSmall) => format!("range range-xs range-{}", type_class),
        Some(ComponentSize::Small) => format!("range range-sm range-{}", type_class),
        Some(ComponentSize::Base) => format!("range range-md range-{}", type_class),
        Some(ComponentSize::Large) => format!("range range-lg range-{}", type_class),
    };

    view! {
        <div class = "flex flex-grow justify-between my-2 font-extralight">
            <label class = format!("block font-light {}", text_class)>{label.clone()}</label>
            <span class=format!("indicator-item badge badge-{}", type_class)>{move || value_signal.get()}</span>
        </div>

        <input
            class=input_class
            type="range"
            name=name.clone()
            prop:step=step
            prop:value=value_signal
            prop:min=min
            prop:max=max
            on:change = move |e| {
                let val = event_target_value(&e).parse::<f64>().unwrap_or_default();
                value_signal.set(val);
            }
        />

        <div class = format!("w-full flex flex-grow justify-between font-light {}", text_class)>
            <span>{format!("{min}")}</span>
            <span>{format!("{max}")}</span>
        </div>
    }
}
