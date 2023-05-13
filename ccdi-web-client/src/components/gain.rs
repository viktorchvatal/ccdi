use yew::{Properties, Callback};
use super::*;

// ============================================ PUBLIC =============================================

pub struct GainSelector;

pub enum Msg {
    SetGain(u16),
}

#[derive(Clone, PartialEq, Properties)]
pub struct GainData {
    pub gain_changed: Callback<u16>,
    pub selected_gain: u16,
}

impl Component for GainSelector {
    type Message = Msg;
    type Properties = GainData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetGain(value) => {
                ctx.props().gain_changed.emit(value)
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let selected = ctx.props().selected_gain;

        html! {
            <div>
                <p>{"Set camera gain"}</p>
                { gain_button(selected, 0, ctx)}
                { gain_button(selected, 1000, ctx)}
                { gain_button(selected, 2000, ctx)}
                { gain_button(selected, 3000, ctx)}
                { gain_button(selected, 4000, ctx)}
            </div>
        }
    }
}

// =========================================== PRIVATE =============================================

fn gain_button(
    current: u16,
    value: u16,
    ctx: &Context<GainSelector>
) -> Html {
    let gain_click = |action: u16| ctx.link().callback(move |_| Msg::SetGain(action));

    let selected_class = match value == current {
        true => Some("button-selected"),
        false => None,
    };

    html! {
        <button
            class={classes!("short-button", selected_class)}
            onclick={gain_click(value)}
            >{format!("{}", value)}
        </button>
    }
}