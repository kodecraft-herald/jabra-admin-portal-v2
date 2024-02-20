use crate::components::common::components::{common_attributes::ComponentSize, common_icons::Icon};
use leptos::*;

pub fn get_input_size_class(size: ComponentSize, base_class: String) -> String {
    match size {
        ComponentSize::ExtraSmall => format!("{} input-xs", base_class.clone()),
        ComponentSize::Small => format!("{} input-sm", base_class.clone()),
        ComponentSize::Base => format!("{} input-md", base_class.clone()),
        ComponentSize::Large => format!("{} input-lg", base_class.clone()),
    }
}

pub fn join_icon(component: View, icon: String, icon_size: ComponentSize) -> View {
    if icon.is_empty() {
        return component.into_view()
    } else {
        let icon_size = match icon_size {
            ComponentSize::ExtraSmall => "h-3 w-3".to_string(),
            ComponentSize::Small => "h-4 w-4".to_string(),
            ComponentSize::Base => "h-5 w-5".to_string(),
            ComponentSize::Large => "h-6 w-6".to_string(),
        };
        view! {
            <div class="join">
                <div class="rounded-md bg-success join-item flex items-center p-3">
                    <Icon
                        title=icon.clone()
                        size=icon_size
                    />
                </div>
                {component}
            </div>
        }.into_view()
    }
}

pub fn change_day(dt: DateTime<Utc>, new_day: String) -> Option<DateTime<Utc>> {
    // Parse the new_day string to extract year, month, and day
    if let Ok(parsed_date) = NaiveDate::parse_from_str(&new_day, "%Y-%m-%d") {
        // Extract the time components from the original DateTime
        let time_components = dt.time();

        // Create a new DateTime with the parsed date and extracted time components
        let new_datetime =
            DateTime::from_naive_utc_and_offset(parsed_date.and_time(time_components), Utc);
        // println!("New datetime: {}", new_datetime);
        Some(new_datetime)
    } else {
        None
    }
}

pub fn extract_date_as_string(dt: DateTime<Utc>) -> String {
    // Get the date part from the DateTime
    let date_part = dt.date_naive();

    // Format the date part into a string
    let formatted_date = format!("{}", date_part);
    println!("Formatted exp: {}", formatted_date);
    formatted_date
}