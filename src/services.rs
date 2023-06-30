use gloo_net::http::Request;
use yew::UseStateHandle;

use crate::entities::Report;

pub fn load_latest_report(
    report: &UseStateHandle<Option<Report>>,
    selected_test_name: &UseStateHandle<String>,
) {
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
}
