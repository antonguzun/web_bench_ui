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
pub struct Root {
    pub created_at: String,
    pub results: Vec<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
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

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    videos.iter().map(|video| {
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| {
                on_click.emit(video.clone())
            })
        };

        html! {
            <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }).collect()
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let videos = use_state(|| None);
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Root = Request::get("/web_bench/temp")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    videos.set(Some(fetched_videos));
                });
                || ()
            },
            (),
        );
    }
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                // <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
                // { for details }
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
