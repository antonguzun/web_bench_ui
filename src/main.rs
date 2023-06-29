use gloo_net::http::Request;
use itertools::Itertools;
use web_bench_ui::components::barchart::BarChart;
use web_bench_ui::components::table::BenchTable;
use web_bench_ui::components::table::TestingResultList;
use web_bench_ui::components::tabs::TestNameTabs;
use web_bench_ui::entities::Report;
use web_sys;
use yew::classes;
use yew::function_component;
use yew::html;
use yew::use_effect_with_deps;
use yew::use_state;
use yew::Callback;
use yew::Html;

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

    let mut table_size = 0;
    let on_tab_select = {
        let selected_test_name = selected_test_name.clone();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        table_size = match document.get_element_by_id("lolkek") {
            Some(x) => x.client_width(),
            None => 0,
        };
        Callback::from(move |test_name: String| selected_test_name.set(test_name))
    };

    // client_width();

    html! {
        <>
            <div class={classes!("center")}>
                <div class={classes!("row")}>
                    <h1>{ "Web servers bench" }</h1>

                    {created_at}

                    <br/>
                    {table_size}
                    <br/>

                    <TestNameTabs test_names={(*test_names).clone()} on_click={on_tab_select.clone()} />
                </div>

                <div style="display: block; margin: auto">
                    <BenchTable>
                        <TestingResultList id={"lolkek"} results={testing_results.clone()} selected_test_name={(*selected_test_name).clone()} />
                    </BenchTable>

                    <BarChart results={testing_results.clone()} selected_test_name={(*selected_test_name).clone()}></BarChart>
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
