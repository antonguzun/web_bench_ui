use std::cmp::max;
use std::cmp::min;

use itertools::Itertools;
use yew::classes;
use yew::function_component;
use yew::html;
use yew::Html;
use yew::Properties;

use crate::entities::TestingResult;
use crate::utils;

#[derive(Properties, PartialEq)]
pub struct BarChartProps {
    pub results: Vec<TestingResult>,
    pub selected_test_name: String,
}

#[function_component(BarChart)]
pub fn bar_chart(props: &BarChartProps) -> Html {
    let fill_color = "fill: rgb(152, 171, 197)";

    let x_max_len = 1000.0;
    let local_x_offset = 70.0;

    let max_bar_width = 55.0;
    let min_bar_width = 50.0;
    let bars_number = props
        .results
        .iter()
        .filter(|r| r.test_name == props.selected_test_name)
        .count();

    let bar_width = {
        let bar_width = (x_max_len - local_x_offset) / (max(bars_number, 1) as f64 * 1.1);
        if bar_width > max_bar_width {
            max_bar_width
        } else if bar_width < min_bar_width {
            min_bar_width
        } else {
            bar_width
        }
    };

    let space_width = bar_width * 0.1;
    let max_height = 270.0;

    let max_rps = match &props
        .results
        .iter()
        .filter(|r| r.test_name == props.selected_test_name)
        .max_by_key(|v| v.requests_per_second as i32)
    {
        Some(v) => v.requests_per_second,
        None => return html! {},
    };
    let max_y_value = utils::round_to_digit(max_rps, 2);

    let y_axis_range = (0..max_y_value + 1).step_by((max_y_value / 10) as usize);
    let one_percent = max_height / max_y_value as f64;
    let y_axis_x_offset = 13.0;
    let tick_len = y_axis_x_offset / 2.0;
    let axis_space_width = y_axis_x_offset / 4.0;

    let mut x_languages: Vec<Html> = vec![];
    let mut x_webservers: Vec<Html> = vec![];
    let mut x_databases: Vec<Html> = vec![];
    let mut x_extra_values: Vec<Html> = vec![];
    let mut bars: Vec<Html> = vec![];

    for (i, result) in props
        .results
        .iter()
        .filter(|r| r.test_name == props.selected_test_name)
        .sorted_by_key(|r| -(r.requests_per_second as i64))
        .enumerate()
    {
        let rect_height = result.requests_per_second * one_percent;
        let y = max_height - rect_height;
        let x = i as f64 * (bar_width + space_width);

        let x_lang = html! {
            <g class="tick" style="opacity: 0.7;" transform="translate(100,0)">
                <line y2="6" x1={format!{"{}", x}} x2={format!{"{}", x}}></line>
                <text
                 dy=".5em" style="text-anchor: middle;" y="11" x={format!{"{}", x}}>
                    {result.language.clone()}
                </text>
            </g>
        };
        x_languages.push(x_lang);

        let x_ws = html! {
            <g class="tick" style="opacity: 0.7;" transform="translate(100,0)">
                <text
                     dy=".5em" style="text-anchor: middle;" y="25" x={format!{"{}", x}}>
                    {result.webserver_name.split('[').next().unwrap()}
                </text>
            </g>
        };
        x_webservers.push(x_ws);

        let x_db = html! {
            if let Some(database) = &result.database {
                <g class="tick" style="opacity: 0.7;" transform="translate(100,0)">
                    <text
                         dy=".5em" style="text-anchor: middle;" y="38" x={format!{"{}", x}}>
                        {database.split('[').next().unwrap()}
                    </text>
                </g>
            }
        };
        x_databases.push(x_db);

        let x_extra_v = html! {
            if let Some(database) = &result.database {
                if let Some(tail) = &database.split('[').last() {

                    <g class="tick" style="opacity: 0.7;" transform="translate(100,0)">
                        <text
                             dy=".5em" style="text-anchor: middle;" y="50" x={format!{"{}", x}}>
                                {
                                    match tail.replace("]", "").len() >= 10 {
                                        true  => format!("{}..", &tail.replace("]", "")[0..min(8,tail.len() - 2)].to_owned()),
                                        false  => tail.replace("]", "").to_owned().to_owned(),
                                    }
                                }
                        </text>
                    </g>
                }
            }
        };
        x_extra_values.push(x_extra_v);

        let bar = html! {
            <rect
                style={fill_color}
                width={format!("{}", bar_width)}
                x={format!{"{}", x+local_x_offset + 3.0}}
                y={format!{"{}", y}}
                height={format!{"{}", rect_height}}>
            </rect>
        };
        bars.push(bar);
    }

    html! {
        <>
            <svg viewBox="0 0 1000 350">
                    <g class="x axis_lang" transform="translate(0,280)">
                        {x_languages}
                    </g>
                    <g class="x axis_ws" transform="translate(0,280)">
                        {x_webservers}
                    </g>
                    <g class="x axis_ws" transform="translate(0,280)">
                        {x_databases}
                    </g>
                    <g class="x axis_ws" transform="translate(0,280)">
                        {x_extra_values}
                    </g>

                    <g transform="translate(0, 10)">

                    <line
                        x1={format!("{}", local_x_offset - y_axis_x_offset)}
                        y1="0"
                        x2={format!("{}", local_x_offset - y_axis_x_offset)}
                        y2={format!("{}", max_height)}>
                    </line>

                    <g class="y axis">
                        {
                            y_axis_range
                                .map(|y| {
                                    let value_height = max_height / max_y_value as f64 * (max_y_value-y) as f64;
                                    html! {
                                        <g transform={format!("translate(0,{})", value_height)}>
                                            <line
                                                x1={format!("{}", local_x_offset - y_axis_x_offset - tick_len)}
                                                y1="0"
                                                x2={format!("{}", local_x_offset - y_axis_x_offset)}
                                                y2="0">
                                            </line>
                                            <text
                                                class={classes!("small_svg_text")}
                                                style="text-anchor: end"
                                                x={format!("{}", local_x_offset - y_axis_x_offset - tick_len - 0.02 * local_x_offset)}
                                                y="2">
                                                    {y}
                                            </text>
                                        </g>
                                    }
                                })
                                .collect::<Html>()
                         }
                    <text
                        class={classes!("small_svg_text")}
                        transform="rotate(-90)"
                        y={format!("{}", local_x_offset - axis_space_width + 2.0)}
                        x={format!("{}", axis_space_width)}
                        style="text-anchor: end">
                            {"Rows / sec"}
                    </text>
                    </g>

                    {bars}

                </g>
            </svg>
        </>
    }
}
