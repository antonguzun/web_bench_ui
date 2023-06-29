use itertools::Itertools;
use yew::classes;
use yew::function_component;
use yew::html;
use yew::Children;
use yew::Component;
use yew::Context;
use yew::Html;
use yew::Properties;

use crate::entities::TestingResult;

#[derive(Properties, PartialEq)]
pub struct BenchTableProps {
    pub children: Children,
}

#[function_component]
pub fn BenchTable(props: &BenchTableProps) -> Html {
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
pub struct TestingResultProps {
    pub results: Vec<TestingResult>,
    pub selected_test_name: String,
}

pub struct TestingResultList;

impl Component for TestingResultList {
    type Message = ();
    type Properties = TestingResultProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let selected_test_name = &ctx.props().selected_test_name;
        let res = ctx.props()
            .results
            .iter()
            .filter(|r| &r.test_name == selected_test_name)
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
            .collect();
        res
    }
}
