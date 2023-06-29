use yew::function_component;
use yew::html;
use yew::Callback;
use yew::Html;
use yew::Properties;
use yew::classes;


#[derive(Properties, PartialEq)]
pub struct TestNameTabsProps {
    pub test_names: Vec<String>,
    pub on_click: Callback<String>,
}

#[function_component(TestNameTabs)]
pub fn test_tabs(props: &TestNameTabsProps) -> Html {
    let on_click = props.on_click.clone();
    props
        .test_names
        .iter()
        .map(|test_name| {
            let on_tab_select = {
                let on_click = on_click.clone();
                let test_name = test_name.clone();
                Callback::from(move |_| {
                    on_click.emit(test_name.clone())
                })
            }; 
            html! {
               <button id={test_name.clone()} class={classes!("tab_btn")} onclick={&on_tab_select.clone()}>{test_name.replace("_", " " )}</button>
            }
        })
        .collect()
}
