use std::sync::Arc;

use ccdi_imager_interface::{ImagerProperties, DeviceProperty};
use yew::Properties;
use super::*;

// ============================================ PUBLIC =============================================

pub struct CameraDetail;

#[derive(Clone, PartialEq, Properties)]
pub struct CameraDetailData {
    pub data: Option<Arc<ImagerProperties>>,
}

impl Component for CameraDetail {
    type Message = ();
    type Properties = CameraDetailData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div class="div-table">
                {render_rows(ctx)}
            </div>
        }
    }
}

fn render_rows(ctx: &Context<CameraDetail>) -> Html {
    ctx.props().data.as_ref().map(|properties| &properties.other).unwrap_or(&Vec::new())
        .iter()
        .map(|property| render_item(property))
        .collect::<Html>()
}

fn render_item(property: &DeviceProperty) -> Html {
    html! {
        <div class="div-table-row">
            <div class="div-table-col">{&property.name}</div>
            <div class="div-table-col">{&property.value}</div>
        </div>
    }
}