use yew::prelude::*;
use yew_infinite_for::InfiniteFor;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    item_num: usize,
}

#[derive(Debug, Clone)]
struct User {
    username: String,
    date: String,
}

#[function_component(Item)]
fn item(props:&Props) -> Html {
    let req_state = use_state(|| -> Option<User> { None });
    let item_num = props.item_num;

    {
        let req_state = req_state.clone();

        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                // Use timeout to simulate latency from async requests
                gloo::timers::callback::Timeout::new(2000, move || {
                    req_state.set(Some(User {
                        username: format!("Quin{item_num}{item_num}"),
                        date: "2022/2/22".to_owned(),
                    }))
                }).forget();
            });
            || ()
        })
    };

    if let Some(req_state) = (*req_state).clone() {
        html! {
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
                <h1>
                    {format!("{}, {}",req_state.username,req_state.date)}
                </h1>
            </div>
        }
    } else {
        html! {
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
            ></div>
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    let request = Callback::from(|(n, ret): (usize, Callback<Html>)| {
        gloo::console::log!("Tag ", n, " is rendered!");
        ret.emit(html!(
            <Item item_num={n}/>
        ))
    });

    html! {
        <div
            style="\
                height: 99vh;\
                width: 100wh;\
            "
        >
            <InfiniteFor {request} >
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
