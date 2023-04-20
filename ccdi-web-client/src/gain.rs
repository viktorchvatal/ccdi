use yew::{Properties, Callback};
use super::*;

// ============================================ PUBLIC =============================================

pub struct GainSelector {
    gain: u16
}

pub enum Msg {
    SetGain(u16),
}

#[derive(Clone, PartialEq, Properties)]
pub struct GainData {
    pub gain_changed: Callback<u16>,
}

impl Component for GainSelector {
    type Message = Msg;
    type Properties = GainData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { gain: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetGain(value) => {
                self.gain = value;
                ctx.props().gain_changed.emit(value)
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{"Set camera gain"}</p>
                { gain_button(self.gain, 0, ctx)}
                { gain_button(self.gain, 1000, ctx)}
                { gain_button(self.gain, 2000, ctx)}
                { gain_button(self.gain, 3000, ctx)}
                { gain_button(self.gain, 4000, ctx)}
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