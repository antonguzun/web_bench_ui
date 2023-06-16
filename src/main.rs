use gloo_net::http::Request;
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
    // on_click: Callback<Video>,
}

#[function_component(TestingResultList)]
fn videos_list(props: &TestingResultProps) -> Html {
    // let on_click = on_click.clone();
    props.results.iter().map(|result| {
        // let on_video_select = {
        //     let on_click = on_click.clone();
        //     let video = video.clone();
        //     Callback::from(move |_| {
        //         on_click.emit(video.clone())
        //     })
        // };

        html! {
            <p key={result.test_name.clone()} >{format!("{}: {}", result.webserver_name, result.language)}</p>
        }
    }).collect()
}

// #[derive(Properties, PartialEq)]
// struct VideosDetailsProps {
//     video: Video,
// }

// #[function_component(VideoDetails)]
// fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
//     html! {
//         <div>
//             <h3>{ video.title.clone() }</h3>
//             <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
//         </div>
//     }
// }

#[function_component]
fn App() -> Html {
    let report = use_state(|| None);
    {
        let report = report.clone();
        use_effect_with_deps(
            move |_| {
                let report = report.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_report: Report = Request::get("/web_bench/temp")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
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

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                {created_at}
                <TestingResultList results={testing_results} />
                // <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
                // { for details }
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
