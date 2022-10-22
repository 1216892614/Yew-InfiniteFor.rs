use web_sys::HtmlDivElement;
use yew::prelude::*;

static mut RENDER_LIST: Option<Html> = None;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// This callback accept a Two-tuple:
    /// - **n: usize:** the serial number of the item
    /// - **ret: Callback<Html>:** Html setter of the item
    pub request: Option<Callback<(usize, Callback<Html>)>>,
    pub children: Option<Children>,
    pub is_direction_row: Option<bool>,
}

/// A list component that scrolls infinitely for Yew.
///
/// # Style to Overflow
///
/// You have to be sure `InfiniteFor` can be overflow.
/// Generally this is not a problem in other components.
/// If you plan to put `InfiniteFor` directly in the
/// `body`, you can use a style like this:
/// 
/// ```ignore
/// html! {
///     <div
///         // You have to be sure `InfiniteFor` can be overflow.
///         // Generally this is not a problem in other components.
///         // If you plan to put `InfiniteFor` directly in the
///         // `body`, you can use a style like this:
///         style="\
///             height: 99vh;\
///             width: 100wh;\
///         "
///     >
///         <InfiniteFor/>
///     </div>
/// }
/// ```
/// 
/// # Example
/// 
/// ```ignore
/// #[function_component(App)]
/// fn app() -> Html {
///     let request = Callback::from(|(n, ret): (usize, Callback<Html>)| {
///         ret.emit(html!(
///             <h1>
///                 {format!("This is tag No.{n}")}
///             </h1>
///         ))
///     });
/// 
///     html! {
///         <InfiniteFor
///             // use this attribute to switch mode from column to row
///             is_direction_row={true}
///             // use this callback to offer Html of the items
///             {request}
///         >
///             // children is the load sign
///             // at the bottom of the page
///             <h4>
///                 {"The end of page!"}
///             </h4>
///         </InfiniteFor>
///     }
/// }
/// ```
#[function_component(InfiniteFor)]
pub fn infinite_for(props: &Props) -> Html {
    let request = props
        .request
        .clone()
        .unwrap_or(Callback::from(|i: (usize, Callback<Html>)| {
            i.1.emit(html!(
                <div
                    style="\
                        min-height: 300px;\
                        min-width: 700px;\
                        height: 300px;\
                        width: 700px;\
                        border-radius: 5px;\
                        background-color: rgba(222, 240, 255, 0.262);\
                        margin: 5px;\
                    "
                >
                    <h1>{i.0}</h1>
                </div>
            ))
        }));

    let loading_block = props.children.clone();
    let request_number_state = use_state(|| 0usize);
    let request_times_state = use_state(|| 0usize);
    let list_state = use_state(|| html!(<></>));
    let is_bottom_state = use_state(|| true);

    let is_direction_row = props.is_direction_row;
    let list_ref = NodeRef::default();
    let loading_block_ref = NodeRef::default();

    let list_pust = {
        Callback::from(move |item: Html| unsafe {
            RENDER_LIST = Some(
                [RENDER_LIST.clone().unwrap_or(html!(<></>)), item]
                    .into_iter()
                    .collect(),
            )
        })
    };

    {
        let list_ref = list_ref.clone();
        let request = request.clone();
        let list_pust = list_pust.clone();
        let list_state = list_state.clone();
        let request_number_state = request_number_state.clone();
        let request_times_state = request_times_state.clone();

        use_effect(move || {
            let list_ref = list_ref.cast::<HtmlDivElement>().unwrap();

            if list_ref.scroll_height() - 100 < list_ref.client_height()
                && list_ref.scroll_width() - 100 < list_ref.client_width()
            {
                request_number_state.set(*request_number_state + 1);
                request_times_state.set(*request_times_state + 1);

                let key = *request_times_state;

                request.emit((key, list_pust));
                unsafe { list_state.set(RENDER_LIST.clone().unwrap_or(html!(<></>))) }
            }
            || ()
        })
    };

    let onscroll = {
        let list_ref = list_ref.clone();
        let loading_block_ref = loading_block_ref.clone();
        let is_bottom_state = is_bottom_state.clone();

        let list_pust = list_pust.clone();
        let request = request.clone();
        let list_state = list_state.clone();
        let request_number_state = request_number_state.clone();
        let request_times_state = request_times_state.clone();

        Callback::from(move |_| {
            let list_ref = list_ref.cast::<HtmlDivElement>().unwrap();
            let list_pos = (list_ref.scroll_left(), list_ref.scroll_top());

            let refresh_range = loading_block_ref.cast::<HtmlDivElement>().unwrap();

            let refresh_range = if is_direction_row.unwrap_or(false) {
                refresh_range.scroll_width()
            } else {
                refresh_range.scroll_height()
            };

            let befor_is_bottom_state = *is_bottom_state;

            let after_is_bottom_state = match (
                is_direction_row,
                list_ref.scroll_width() - list_pos.0 - list_ref.client_width() <= refresh_range,
                list_ref.scroll_height() - list_pos.1 - list_ref.client_height() <= refresh_range,
            ) {
                (Some(true), true, _) | (None, _, true) | (Some(false), _, true) => true,
                _ => false,
            };

            if !befor_is_bottom_state && after_is_bottom_state {
                request_times_state.set(*request_times_state + *request_number_state);

                for i in 0..*request_number_state {
                    let key = *request_times_state + i;

                    request.emit((key, list_pust.clone()));
                    unsafe { list_state.set(RENDER_LIST.clone().unwrap_or(html!(<></>))) }
                }
            }

            list_ref.scroll_to_with_x_and_y(list_pos.0 as f64, list_pos.1 as f64);

            is_bottom_state.set(after_is_bottom_state);
        })
    };

    html! {
        <>
            <div
                ref={list_ref}
                {onscroll}
                style={
                    format!("\
                        height: 100%;\
                        width: 100%;\
                        margin-bottom: 5px;\
                        display: flex;\
                        justify-content: flex-start;\
                        align-items: center;\
                        flex-direction: {};\
                        overflow: auto;\
                    ",
                    if is_direction_row == Some(true) {"row"}
                    else {"column"})
                }
            >
                //======List=======
                {(*list_state).clone()}
                //==Loading Block==
                <div
                    ref={loading_block_ref}
                    style="\
                        width: auto;\
                        height: auto;\
                    "
                >
                    {
                        if let Some(loading_block) = loading_block {
                            loading_block
                        } else {
                            Children::new(vec![html!(
                                <h4
                                    style="\
                                        background: rgb(18,23,46);\
                                        padding: 5px;\
                                        border-radius: 5px;\
                                    "
                                >
                                    {"到底了"}
                                </h4>
                            )])
                        }
                    }
                </div>
            </div>
        </>
    }
}
