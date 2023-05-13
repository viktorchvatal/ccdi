use ccdi_common::ExposureCommand;
use yew::{Properties, Callback};
use super::*;

// ============================================ PUBLIC =============================================

pub struct CompositionDetail;

#[derive(Clone, PartialEq, Properties)]
pub struct CompositionDetailData {
    pub on_action: Callback<StateMessage>,
    pub camera_params: CameraParams,
}

pub enum Msg{
    ServerAction(StateMessage)
}

impl Component for CompositionDetail {
    type Message = Msg;
    type Properties = CompositionDetailData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }


    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ServerAction(action) => ctx.props().on_action.emit(action),
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        use StateMessage::*;
        use CameraParamMessage::*;

        let loop_enabled = ctx.props().camera_params.loop_enabled;

        let server_action = |action: StateMessage| ctx.link().callback(
            move |_| Msg::ServerAction(action.clone())
        );

        html!{
            <div>
                <div>{"Composition"}</div>
                <button onclick={
                    server_action(ExposureMessage(ExposureCommand::Start))
                }>{"Expose"}</button>
                <button
                    class={classes!(if loop_enabled { Some("button-selected") } else { None })}
                    onclick={server_action(CameraParam(EnableLoop(true)))}
                    >{"Start Loop"}
                </button>
                <button
                    class={classes!(if !loop_enabled { Some("button-selected") } else { None })}
                    onclick={server_action(CameraParam(EnableLoop(false)))}
                    >{"End Loop"}
                </button>
            </div>
        }
    }
}
