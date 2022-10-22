# ✨Yew-InfiniteFor.rs✨

[English](./README.md)

`Yew-InfiniteFor.rs` 是一个 `Yew.rs` 框架上的无限滚动列表组件.

# 🔥为什么 Yew-InfiniteFor.rs 是神🔥

-   它支持 function_component 风格!
-   它可以根据组件视口的大小调整一次渲染的组件数量.
-   它有横纵两种模式.
-   当然, 它的使用很简单!

# 📑如何使用它?📑

只需要进行这样的3步:

1. 获取 `Yew-Canvas.rs`!

    ```toml
    #Cargo.toml
    [dependencies]
    yew="0.19"
    yew-infinite-list="..."
    ```

1. 在 function_component 里创建一个 `request` 回调!

    ```rust
    // 这个回调接受两个参数:
    // - n: usize: 是条目的序号
    // - ret: Callback<Html>: 条目的 Html 构造器
    let request = Callback::from(|(n, ret): (usize, Callback<Html>)| {
        gloo::console::log!("标签 ", n, " 完成渲染!");
        ret.emit(html!(
            <h1>
                {format!("这是标签 {n}")}
            </h1>
        ))
    });
    ```

1. 返回 `InfiniteFor` 组件!
    ```rust
    html! {
        <div
            // 你必须确保 `InfiniteFor` 可以正常溢出.
            // 一般来说, 它在其他已经限制尺寸的组件里不是问题.
            // 但是如果你打算直接在 `body` 里用 `InfiniteFor`
            // 你需要这样的样式:
            style="\
                height: 100vh;\
                width: 100vw;\
            "
        >
            <InfiniteFor
                // 使用这个属性切换至横模式
                is_direction_row={true}
                // 使用这个回调来构造条目的 Html
                {request}
            >
                // 子组件会的列表结尾渲染
                <h4>
                    { "This is the end of page\n这里是页面底部" }
                </h4>
            </InfiniteFor>
        </div>
    }
    ```

# 👌运行实例👌

0. *这一步需要你配置完成 `Yew.rs` 的开发环境, 下面以 `Trunk` 打包器举例:
1. `cd ./examples/base-use`
2. `trunk server`

# ⚖️开源协议⚖️

`Yew-InfiniteFor.rs` 是基于 MIT license 和 the Apache License (Version 2.0) 双协议开源, 使用其中你喜欢的协议来进行开发吧!
