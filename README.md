# ✨Yew-InfiniteFor.rs✨

[中文](https://github.com/1216892614/Yew-InfiniteFor.rs/blob/main/README-zh.md)

`Yew-InfiniteFor.rs` is a list component that scrolls infinitely for Yew.

# 🔥What's the highlight🔥

-   It support function_component style!
-   It can smart change how many items render in once by view size.
-   It support row mode and column mode.
-   And, it's easy to use!

# 📑How to use it?📑

Just 3 Simple steps, do like this:

1. Get `Yew-Canvas.rs`!

    ```toml
    #Cargo.toml
    [dependencies]
    yew="0.19"
    yew-infinite-list="..."
    ```

1. Create a `request` Callback in the function_component!

    ```rust
    // This callback accept a Two-tuple:
    // - n: usize: the serial number of the item
    // - ret: Callback<Html>: Html setter of the item
    let request = Callback::from(|(n, ret): (usize, Callback<Html>)| {
        gloo::console::log!("Tag ", n, " rendered!");
        ret.emit(html!(
            <h1>
                {format!("This is tag {n}")}
            </h1>
        ))
    });
    ```

1. Return the component as `InfiniteFor`!
    ```rust
    html! {
        <div
            // You have to be sure `InfiniteFor` can be overflow.
            // Generally this is not a problem in other components.
            // If you plan to put `InfiniteFor` directly in the
            // `body`, you can use a style like this:
            style="\
                height: 100vh;\
                width: 100vw;\
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
                    { "This is the end of page\n这里是页面底部" }
                </h4>
            </InfiniteFor>
        </div>
    }
    ```

# 👌Run Exmple👌

0. This requires you to set up the `Yew.rs` development environment in advance, the following is a `Trunk` packaging example:
1. `cd ./examples/base-use`
2. `trunk server`

# ⚖️License⚖️

`Yew-InfiniteFor.rs` is dual licensed under the MIT license and the Apache License (Version 2.0).
