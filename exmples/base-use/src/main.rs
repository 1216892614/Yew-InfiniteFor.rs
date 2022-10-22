use yew::prelude::*;
use yew_infinite_for::InfiniteFor;

#[function_component(App)]
fn app() -> Html {
    let request = Callback::from(|(n, ret): (usize, Callback<Html>)| {
        gloo::console::log!("Tag ", n, " is rendered!");
        ret.emit(html!(
            <h1>
                {format!("This is tag No.{n}")}
            </h1>
        ))
    });

    html! {
        <div
            // You have to be sure `InfiniteFor` can be overflow.
            // Generally this is not a problem in other components.
            // If you plan to put `InfiniteFor` directly in the
            // `body`, you can use a style like this:
            style="\
                height: 99vh;\
                width: 100wh;\
            "
        >
            <InfiniteFor
                // use this attribute to switch mode from column to row
                is_direction_row={true}
                // use this callback to offer Html of the items
                {request}
            >
                // children is the load sign
                // at the bottom of the page
                <h4>
                    {"The end of page!"}
                </h4>
            </InfiniteFor>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
