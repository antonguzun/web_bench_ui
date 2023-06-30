use itertools::Itertools;
use web_bench_ui::components::barchart::BarChart;
use web_bench_ui::components::table::BenchTable;
use web_bench_ui::components::table::TestingResultList;
use web_bench_ui::components::tabs::TestNameTabs;
use web_bench_ui::services::load_latest_report;
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
            // on first render
            move |_| {
                load_latest_report(&report, &selected_test_name);
            },
            // on last render do nothing
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
        Callback::from(move |test_name: String| selected_test_name.set(test_name))
    };

    html! {
        <>
            <div class={classes!("center")}>
                <div class={classes!("row")}>
                    <h1>{ "Web servers bench" }</h1>

                    {created_at}
                    <br/>

                    <TestNameTabs test_names={(*test_names).clone()} on_click={on_tab_select.clone()} />
                </div>

                <div style="width: fit-content; margin: auto">
                    <BenchTable>
                        <TestingResultList
                            results={testing_results.clone()}
                            selected_test_name={(*selected_test_name).clone()} />
                    </BenchTable>

                    <BarChart results={testing_results.clone()} selected_test_name={(*selected_test_name).clone()}></BarChart>
                </div>
            </div>
             <footer>
              <p>{"Built using "} <a href="https://yew.rs/">{"yew"}</a></p>
              <p><a href="https://github.com/antonguzun/webservers_benchmark">{"GitHub"}</a></p>
            </footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
