use yew::Properties;
use super::*;

// ============================================ PUBLIC =============================================

pub struct CompositionDetail;

#[derive(Clone, PartialEq, Properties)]
pub struct CompositionDetailData {

}

impl Component for CompositionDetail {
    type Message = ();
    type Properties = CompositionDetailData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <div>{"Composition"}</div>
        }
    }
}
