use crate::infrastructure::repository::analysis::http::Http as AnalysisHttpRepository;

use shared::analysis::domain::analysis::Analysis;

use log::{error, info};
use plotly::Plot;
use uuid::Uuid;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

#[hook]
pub fn use_analyses(id: &Uuid, repo: AnalysisHttpRepository) -> SuspensionResult<Option<Analysis>> {
    let result_handle = use_state(|| None);
    let result = (*result_handle).clone();

    let suspension_hanlde = use_state(|| {
        let id = id.to_owned();
        Suspension::from_future(async move {
            match repo.find_by_id(&id).await {
                Ok(Some(found_analysis)) => result_handle.set(Some(found_analysis)),
                Ok(None) => {
                    error!("No analysis found");
                    result_handle.set(None);
                }
                Err(e) => {
                    error!("Error fetching analysis: {e}");
                    result_handle.set(None);
                }
            }
        })
    });

    let suspension = (*suspension_hanlde).clone();
    if suspension.resumed() {
        return result.map_or_else(|| Err(suspension), |v| Ok(Some(v)));
    }
    Err(suspension)
}

#[hook]
pub fn use_plotly_draw(plot: Plot, div_id: String, div_ref: NodeRef) -> SuspensionResult<()> {
    if let Some(element) = div_ref.cast::<HtmlElement>() {
        info!("{element:?} exists!");
    }

    let suspension = Suspension::from_future(async move {
        let div_id = div_id.clone();
        let plot = plot.clone();
        info!("Ready to paint in div `{div_id}`");
        plotly::bindings::new_plot(&div_id, &plot).await;
    });

    if suspension.resumed() {
        Ok(())
    } else {
        error!("Can't complete suspension");
        Err(suspension)
    }
}
