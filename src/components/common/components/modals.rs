use leptos::html::Div;
use leptos::*;

/// Component for a modal that has a confirm and cancel button.
/// Executes a function when the confirm button is clicked.
/// Has a pending state that shows a loading spinner when the action is pending.
///
/// ## Example
/// ```rust
/// let (show_confirm_modal, set_show_confirm_modal) = create_signal(false);
/// let call_action_fn = move || {add_quote_action.dispatch()};
/// let add_quote_action: Action<Vec<Quote>, ()> = create_action(/*action here*/);
///
/// <ConfirmModal
///    when = show_confirm_modal.get()
///    write_signal = set_show_confirm_modal
///    function = call_action_fn
///    action = add_quote_action
/// />
/// ```

#[allow(non_snake_case)]
#[component]
pub fn ConfirmModal<C, T>(
    when: bool,
    write_signal: WriteSignal<bool>,
    mut function: C,
    action: Action<T, ()>,
) -> impl IntoView
where
    C: FnMut() + Clone + 'static,
    T: Clone + 'static,
{
    let on_click = move |_| function();
    let is_pending = action.pending();
    let set_show_modal = write_signal;
    let modal_ref = create_node_ref::<Div>();

    // Checks if an action has a value, then sets the show_modal to false, and resets the action_value to None
    create_effect(move |_| {
        let action_value = action.value();

        if let Some(_action) = action_value() {
            set_show_modal(false);
            action_value.set(None);
        }
    });

    view! {

        <Show when=move || when fallback=|| ()>
            <div node_ref=modal_ref class="modal-cust-top blur-bg">
                <div class="modal-box">
                    <h3 class="font-bold text-2xl">CONFIRM?</h3>
                    <p class="py-4">Are you sure you want to confirm?</p>
                    <div class="modal-action">
                        <button class="btn btn-error btn-sm rounded" prop:disabled=is_pending title="Cancel" on:click = move |_| set_show_modal(false)>Cancel</button>
                        {
                            match is_pending() {
                                true => view! {
                                    <button class="btn btn-success btn-sm rounded" title="Confirm"><span class="loading loading-spinner loading-sm"></span></button>
                                }.into_any(),
                                false => view! {
                                    <button class="btn btn-success btn-sm rounded" title="Confirm" on:click = on_click.clone()>Confirm</button>
                                }.into_any(),
                            }
                        }

                    </div>
                </div>
            </div>
        </Show>

    }
}

/// Component for a confirm modal that is closely tied to the function it is executing.
/// Is used to approve or reject quotes by batch.
/// Executes a function with only **two** parameters.
/// Has a pending signal instead of an action.
///
/// ## Example
/// ```rust
/// let confirm_modal_approve = create_rw_signal(false);
/// let is_pending = approve_reject_quotes_option_action.pending();
/// let call_action_fn = move |String, Vec<String>| {approve_reject_quotes_option_action.dispatch(String, Vec<String>)};
///
/// view! {
///     <ConfirmModalBatchQuotes
///         signal = confirm_modal_approve
///         function = call_action_fn
///         params = ("approved".to_string(), group_ids())
///         pending_signal = is_pending
///         title = "APPROVE".to_string()
///     />
/// }
/// ```

#[allow(non_snake_case)]
#[component]
pub fn ConfirmModalBatchQuotes<C>(
    signal: RwSignal<bool>,
    mut function: C,
    params: (String, Vec<String>),
    pending_signal: ReadSignal<bool>,
    title: String,
) -> impl IntoView
where
    C: FnMut(String, Vec<String>) + Clone + 'static,
{
    let on_click = move |_| function(params.0.clone(), params.1.clone());

    view! {

        <Show when=move || signal.get() fallback=|| ()>
            <div class="modal-cust-top blur-bg">
                <div class="modal-box">
                    <h3 class="font-bold text-2xl">{title.clone()}?</h3>
                    <p class="py-4">Are you sure you want to {title.clone()}?</p>
                    <div class="modal-action">
                        <button class="btn btn-error btn-sm rounded" prop:disabled=pending_signal title="Cancel" on:click = move |_| signal.set(false)>Cancel</button>
                        {
                            match pending_signal() {
                                true => view! {
                                    <button class="btn btn-success btn-sm rounded" title="Confirm"><span class="loading loading-spinner loading-sm"></span></button>
                                }.into_any(),
                                false => view! {
                                    <button class="btn btn-success btn-sm rounded" title="Confirm" on:click = on_click.clone()>Confirm</button>
                                }.into_any(),
                            }
                        }

                    </div>
                </div>
            </div>
        </Show>

    }
}

/// Component for a confirm modal that is closely tied to the function it is executing.
/// Is used to approve or reject all quotes.
/// Executes a function with one parameter.
/// Has a pending signal instead of an action.
///
/// ## Example
/// ```rust
/// let confirm_modal_approve_all = create_rw_signal(false);
/// let is_pending = approve_reject_quotes_option_action.pending();
/// let call_action_fn = move |String| {approve_reject_quotes_option_action.dispatch(String)};
///
/// view! {
///     <ConfirmModalBatchQuotes
///         signal = confirm_modal_approve_all
///         function = call_action_fn
///         params = "approved".to_string()
///         pending_signal = is_pending
///         title = "APPROVE ALL".to_string()
///     />
/// }
/// ```

#[allow(non_snake_case)]
#[component]
pub fn ConfirmModalAllQuotes<C>(
    signal: RwSignal<bool>,
    mut function: C,
    params: String,
    pending_signal: ReadSignal<bool>,
    title: String,
) -> impl IntoView
where
    C: FnMut(String) + Clone + 'static,
{
    let on_click = move |_| function(params.clone());

    view! {

        <Show when=move || signal.get() fallback=|| ()>
            <div class="modal-cust-top blur-bg">
                <div class="modal-box">
                    <h3 class="font-bold text-2xl">{title.clone()}?</h3>
                    <p class="py-4">Are you sure you want to {title.clone()}?</p>
                    <div class="modal-action">
                        <button class="btn btn-error btn-sm rounded" prop:disabled=pending_signal title="Cancel" on:click = move |_| signal.set(false)>Cancel</button>
                        {
                            match pending_signal() {
                                true => view! {
                                    <button class="btn btn-success btn-sm rounded" title="Confirm"><span class="loading loading-spinner loading-sm"></span></button>
                                }.into_any(),
                                false => view! {
                                    <button class="btn btn-success btn-sm rounded" title="Confirm" on:click = on_click.clone()>Confirm</button>
                                }.into_any(),
                            }
                        }

                    </div>
                </div>
            </div>
        </Show>

    }
}

/// Component for a success modal.
/// Has a read and write signal.
/// Has a message of type String.
///
/// ## Example
/// ```rust
/// let (show_success_modal, set_show_success_modal) = create_signal(false);
///
/// view! {
///     <SuccessModal
///        read_signal = show_success_modal
///        write_signal = set_show_success_modal
///        message = "Successfully added quote".to_string()
///     />
/// }
/// ```

#[allow(non_snake_case)]
#[component]
pub fn SuccessModal(
    read_signal: ReadSignal<bool>,
    write_signal: WriteSignal<bool>,
    message: String,
) -> impl IntoView {
    let set_show_modal = write_signal;
    let modal_ref = create_node_ref::<Div>();

    set_show_modal.set(true);

    view! {

        <Show when=move || read_signal.get() fallback=|| ()>
            <div node_ref=modal_ref class="modal-cust-top blur-bg">
                <div class="success-modal-box">
                    <h3 class="font-bold text-2xl text-black">SUCCESS!</h3>
                    <p class="py-4 text-black">{message.clone()}</p>
                    <div class="modal-action">
                        <button class="btn btn-sm rounded" title="Close" on:click = move |_| set_show_modal.set(false)>Close</button>
                    </div>
                </div>
            </div>
        </Show>

    }
}

/// Component for an error modal.
/// Has a read and write signal.
/// Has a message of type String.
///
/// ## Example
/// ```rust
/// let (show_error_modal, set_show_error_modal) = create_signal(false);
///
/// view! {
///     <ErrorModal
///        read_signal = show_error_modal
///        write_signal = set_show_error_modal
///        message = "An unexpected error occurred".to_string()
///     />
/// }
/// ```

#[allow(non_snake_case)]
#[component]
pub fn ErrorModal(
    read_signal: ReadSignal<bool>,
    write_signal: WriteSignal<bool>,
    message: String,
) -> impl IntoView {
    let set_show_modal = write_signal;
    let modal_ref = create_node_ref::<Div>();

    set_show_modal.set(true);

    view! {

        <Show when=move || read_signal.get() fallback=|| ()>
            <div node_ref=modal_ref class="modal-cust-top blur-bg">
                <div class="error-modal-box">
                    <h3 class="font-bold text-2xl text-black">ERROR!</h3>
                    <p class="py-4 text-black">{message.clone()}</p>
                    <div class="modal-action">
                        <button class="btn btn-sm rounded" title="Close" on:click = move |_| set_show_modal.set(false)>Close</button>
                    </div>
                </div>
            </div>
        </Show>

    }
}

/// Component for a success modal that refetches a resource after closing the modal.
/// Has a read and write signal.
/// Has a message of type String.
/// Has a resource with two generic type parameters.
///
/// ## Example
/// ```rust
/// let (show_success_modal, set_show_success_modal) = create_signal(false);
///
/// view! {
///     <SuccessModalWithRefetch
///        read_signal = show_success_modal
///        write_signal = set_show_success_modal
///        message = "Successfully added quote".to_string()
///        resource = active_quotes_resource
///     />
/// }
/// ```

#[allow(non_snake_case)]
#[component]
pub fn SuccessModalWithRefetch<S, T>(
    read_signal: ReadSignal<bool>,
    write_signal: WriteSignal<bool>,
    message: String,
    resource: Resource<S, T>,
) -> impl IntoView
where
    S: Clone + 'static,
    T: 'static,
{
    let set_show_modal = write_signal;
    let modal_ref = create_node_ref::<Div>();

    set_show_modal.set(true);
    let close_and_refetch = move || {
        set_show_modal.set(false);
        resource.refetch();
    };
    view! {

        <Show when=move || read_signal.get() fallback=|| ()>
            <div node_ref=modal_ref class="modal-cust-top blur-bg">
                <div class="success-modal-box">
                    <h3 class="font-bold text-2xl text-black">SUCCESS!</h3>
                    <p class="py-4 text-black">{message.clone()}</p>
                    <div class="modal-action">
                        <button class="btn btn-sm rounded" title="Close" on:click = move |_| close_and_refetch()>Close</button>
                    </div>
                </div>
            </div>
        </Show>

    }
}

/// Component for a success modal that refetches a resource after closing the modal.
/// Similar to [`SuccessModalWithRefetch`] but has a function instead of a resource to allow for more flexibility.
/// Has only a read signal.
/// Has a message of type String.
/// Has a function.
///
/// ## Example
/// ```rust
/// let (show_success_modal, set_show_success_modal) = create_signal(false);
/// let refetch_resource = move || {
/// // Can do anything here unlike SuccessModalWithRefetch which is limited to refetching one resource.
///     set_show_success_modal.set(false);
///     active_quotes_resource.refetch();
/// };
///
/// view! {
///     <SuccessModalRefetch
///        read_signal = show_success_modal
///        message = "Successfully added quote".to_string()
///        resource = refetch_resource
///     />
/// }
/// ```

#[allow(non_snake_case)]
#[component]
pub fn SuccessModalRefetch<F>(
    read_signal: ReadSignal<bool>,
    message: String,
    mut function: F,
) -> impl IntoView
where
    F: FnMut() + Clone + 'static,
{
    let modal_ref = create_node_ref::<Div>();

    let on_click = move |_| {
        function();
    };

    view! {

        <Show when=move || read_signal.get() fallback=|| ()>
            <div node_ref=modal_ref class="modal-cust-top blur-bg">
                <div class="success-modal-box">
                    <h3 class="font-bold text-2xl text-black">SUCCESS!</h3>
                    <p class="py-4 text-black">{message.clone()}</p>
                    <div class="modal-action">
                        <button class="btn btn-sm rounded" title="Close" on:click = on_click.clone()>Close</button>
                    </div>
                </div>
            </div>
        </Show>

    }
}
