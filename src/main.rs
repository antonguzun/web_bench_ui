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
use web_bench_ui::utils;

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
                    <td class={classes!("right_align", "font-variant-numeric")}>{format!("{:.2}", &result.requests_per_second)}</td>
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

#[derive(Properties, PartialEq)]
struct BarChartProps{
    results: Vec<TestingResult>,
    selected_test_name: String,
}



#[function_component(BarChart)]
fn bar_chart(props: &BarChartProps) -> Html {
    let fill_color = "fill: rgb(152, 171, 197)";
    let bar_width = 35.0;
    let space_width = 4.0;
    let max_height = 250.0;

    let x_offset = 105.0;
    let max_rps = match &props.results.iter().filter(|r| r.test_name == props.selected_test_name).max_by_key(|v| v.requests_per_second as i32){
        Some(v) => v.requests_per_second,
        None => return html!{},
    };
    let max_y_value = utils::round_to_digit(max_rps, 2);

    let y_axis_range = (0..max_y_value + 1 ).step_by((max_y_value / 10) as usize);
    let one_percent = max_height / max_y_value as f64;
    let y_axis_x_offset = x_offset * 0.1;
    let tick_len = y_axis_x_offset / 2.0;
    let axis_space_width = y_axis_x_offset / 4.0;
    html!{
            <svg style="width: 90%" viewBox="0 0 800 330" >
                <g transform="translate(0, 10)">
                    <line x1={format!("{}", x_offset - y_axis_x_offset)} y1="0" x2={format!("{}", x_offset - y_axis_x_offset)} y2={format!("{}", max_height)}></line>
                    <g class="y axis">
                        {
                            y_axis_range
                                .map(|y| {
                                    let value_height = max_height / max_y_value as f64 * (max_y_value-y) as f64;
                                    html! {
                                        <g transform={format!("translate(0,{})", value_height)}>
                                            <line x1={format!("{}", x_offset - y_axis_x_offset - tick_len)} y1="0" x2={format!("{}", x_offset - y_axis_x_offset)} y2="0"></line>
                                            <text class={classes!("small_svg_text")} style="text-anchor: end" x={format!("{}", x_offset - y_axis_x_offset - tick_len - 0.02 * x_offset)} y="2">{y}</text>
                                        </g>
                                    }
                                })
                                .collect::<Html>()
                         }
                    <text class={classes!("small_svg_text")} transform="rotate(-90)" y={format!("{}", x_offset - axis_space_width)} x={format!("{}", axis_space_width * -1.0)} style="text-anchor: end">{"Rows / sec"}</text>
                    </g>
                    {
                        props
                            .results
                            .iter()
                            .filter(|r| r.test_name == props.selected_test_name)
                            .sorted_by_key(|r| -(r.requests_per_second as i64))
                            .enumerate()
                            .map(|(i, result)| {
                                let rect_height = result.requests_per_second * one_percent;
                                let y = max_height - rect_height;
                                let x = i as f64 * (bar_width + space_width);
                                html! {
                                    <rect 
                                        style={fill_color} 
                                        width={format!("{}", bar_width)} 
                                        x={format!{"{}", x+x_offset}} 
                                        y={format!{"{}", y}} 
                                        height={format!{"{}", rect_height}}>
                                    </rect>
                                }
                            })
                            .collect::<Html>()
                     }
                </g>
            </svg>
    }
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
                <div class={classes!("row")}>
                    <h1>{ "Web servers bench" }</h1>

                    {created_at}
                    <br/>

                    <TestNameTabs test_names={(*test_names).clone()} on_click={on_tab_select.clone()} />
                </div>

                <div style="display: block; margin: auto">
                    <BenchTable>
                        <TestingResultList results={testing_results.clone()} selected_test_name={(*selected_test_name).clone()} />
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
