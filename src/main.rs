use gloo_net::http::Request;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use yew::function_component;
use yew::html;
use yew::use_effect_with_deps;
use yew::use_state;
use yew::Callback;
use yew::Html;
use yew::Properties;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Report {
    pub created_at: String,
    pub results: Vec<TestingResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestingResult {
    pub test_name: String,
    pub webserver_name: String,
    pub language: String,
    pub database: Option<String>,
    pub orm: Option<String>,
    pub requests_per_second: f64,
    pub latency_p50: String,
    pub latency_p75: String,
    pub latency_p90: String,
    pub latency_p99: String,
}

#[derive(Properties, PartialEq)]
struct TestingResultProps {
    results: Vec<TestingResult>,
    selected_test_name: String,
}

#[function_component(TestingResultList)]
fn test_result_list(props: &TestingResultProps) -> Html {
    props
        .results
        .iter()
        .filter(|r| r.test_name == props.selected_test_name)
        .sorted_by_key(|r| -(r.requests_per_second as i64))
        .map(|result| {
            html! {
                <>
                  <tr>
                    <td>{&result.webserver_name}</td>
                    <td>{&result.language}</td>
                    <td>{&result.database.clone().unwrap_or("no db".to_owned())}</td>
                    <td>{&result.orm.clone().unwrap_or("no orm".to_owned())}</td>
                    <td>{&result.requests_per_second}</td>
                    <td>{&result.latency_p50}</td>
                    <td>{&result.latency_p75}</td>
                    <td>{&result.latency_p90}</td>
                    <td>{&result.latency_p99}</td>
                  </tr>
                </>
            }
        })
        .collect()
}

#[derive(Properties, PartialEq)]
struct TestNameTabsProps {
    test_names: Vec<String>,
    on_click: Callback<String>,
}

#[function_component(TestNameTabs)]
fn test_tabs(props: &TestNameTabsProps) -> Html {
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
               <>
                    <button id={test_name.clone()} class="tablinks" onclick={&on_tab_select.clone()}>{test_name.replace("_", " " )}</button>
               </>
            }
        })
        .collect()
}

#[function_component]
fn App() -> Html {
    let selected_test_name = use_state(|| "not existed testname".to_owned());
    let report = use_state(|| None);
    {
        let report = report.clone();
        let selected_test_name = selected_test_name.clone();
        use_effect_with_deps(
            move |_| {
                let report = report.clone();
                let selected_test_name = selected_test_name.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_report: Report = Request::get("/web_bench/temp")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    if fetched_report.results.len() > 0 {
                        selected_test_name.set(fetched_report.results[0].test_name.clone());
                    }

                    report.set(Some(fetched_report));
                });
                || ()
            },
            (),
        );
    }
    let created_at = match &*report {
        Some(v) => v.created_at.clone(),
        None => "Not found".to_owned(),
    };
    let testing_results = match &*report {
        Some(v) => v.results.clone(),
        None => vec![],
    };
    let test_names = &testing_results
        .iter()
        .map(|x| x.test_name.to_owned())
        .unique()
        .collect::<Vec<String>>();
    let on_tab_select = {
        let selected_test_name = selected_test_name.clone();
        Callback::from(move |test_name: String| {
            selected_test_name.set(test_name)
        })
    };

    html! {
        <>
            <h1>{ "Web bench" }</h1>
            <div>
                <h3>{"test cases"}</h3>
                {created_at}
               <br/>

                <TestNameTabs test_names={(*test_names).clone()} on_click={on_tab_select.clone()} />

                <TestingResultList results={testing_results} selected_test_name={(*selected_test_name).clone()} />
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
