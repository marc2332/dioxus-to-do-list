use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let list = use_state(&cx, || vec!["Sample".to_string()]);
    let list_len = list.len();
    let new_item = use_state(&cx, || "".to_string());

    cx.render(rsx!(
        h1 { "List length: {list_len}" }
        input {
            oninput: move |evt| new_item.set(evt.value.clone()),
        }
        button { onclick: move |_| {
            let item = new_item.get().to_string();
            let items_list = list.get().clone();

            list.set([items_list, vec![item]].concat());
            new_item.set("".to_string());

        }, "Add" }
        div {
            list.iter().enumerate().map(|(i, item)| {
                let item_name = item.clone();
                rsx! {
                    li {
                        "[{i}] {item_name}"
                    }
                }
            })
        }
    ))
}
