use gloo_net::http::Request;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use yew::Children;
use yew::function_component;
use yew::html;
use yew::use_effect_with_deps;
use yew::use_state;
use yew::Callback;
use yew::Html;
use yew::Properties;
use yew::classes;

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
pub struct BenchTableProps {
    pub children: Children,
}

#[function_component]
fn BenchTable(props: &BenchTableProps) -> Html {
    html! {
        <div>
        <table class={classes!("center_t")}>
            <thead>
              <tr class={classes!("first_line")}>
                <td class={classes!("left_align")}>{"Web Server"}</td>
                <td class={classes!("left_align")}>{"Language"}</td>
                <td class={classes!("left_align")}>{"Database"}</td>
                <td class={classes!("left_align")}>{"ORM"}</td>
                <td class={classes!("right_align")}>{"RPS"}</td>
                <td class={classes!("right_align")}>{"Latency p50"}</td>
                <td class={classes!("right_align")}>{"Latency p75"}</td>
                <td class={classes!("right_align")}>{"Latency p90"}</td>
                <td class={classes!("right_align")}>{"Latency p99"}</td>
              </tr>
            </thead>
            <tbody>
                { for props.children.iter() }
            </tbody>
        </table>
        </div>

    }
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
                    <td class={classes!("left_align")}>{&result.webserver_name}</td>
                    <td class={classes!("left_align")}>{&result.language}</td>
                    <td class={classes!("left_align")}>{&result.database.clone().unwrap_or("no db".to_owned())}</td>
                    <td class={classes!("left_align")}>{&result.orm.clone().unwrap_or("no orm".to_owned())}</td>
                    <td class={classes!("right_align", "font-variant-numeric")}>{&result.requests_per_second}</td>
                    <td class={classes!("right_align")}>{&result.latency_p50}</td>
                    <td class={classes!("right_align")}>{&result.latency_p75}</td>
                    <td class={classes!("right_align")}>{&result.latency_p90}</td>
                    <td class={classes!("right_align")}>{&result.latency_p99}</td>
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
               <button id={test_name.clone()} class={classes!("tab_btn")} onclick={&on_tab_select.clone()}>{test_name.replace("_", " " )}</button>
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
                    let fetched_report: Report = Request::get("/web_benchmark/reports/latest")
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
            <div class={classes!("center")}>
                <h1>{ "Web servers bench" }</h1>

                {created_at}
                <br/>

                <TestNameTabs test_names={(*test_names).clone()} on_click={on_tab_select.clone()} />
            </div>
            <BenchTable>
                <TestingResultList results={testing_results} selected_test_name={(*selected_test_name).clone()} />
            </BenchTable>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
