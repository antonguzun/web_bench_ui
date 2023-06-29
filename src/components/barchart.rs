use std::cmp::max;

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
    let min_bar_width = 15.0;
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
    html! {
        <>
            // {format!("t_w:{} w_w:{} offset_x:{}", table_size, window_width, x_offset)}
            // <svg style="90%" viewBox="0 0 800 330" >
            <svg viewBox="0 0 1000 330">
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
                                        x={format!{"{}", x+local_x_offset + 3.0}}
                                        y={format!{"{}", y}}
                                        height={format!{"{}", rect_height}}>
                                    </rect>
                                }
                            })
                            .collect::<Html>()
                     }
                </g>
            </svg>
        </>
    }
}
