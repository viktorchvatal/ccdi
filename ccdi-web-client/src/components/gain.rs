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
    pub config: ButtonSet<u16>,
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
        let buttons = &ctx.props().config;

        html! {
            <div>
                <p>{"Set camera gain"}</p>
                {render_buttons(buttons, selected, ctx)}
            </div>
        }
    }
}

// =========================================== PRIVATE =============================================

fn render_buttons(
    button_set: &ButtonSet<u16>,
    current: u16,
    ctx: &Context<GainSelector>
) -> Html {
    button_set.buttons.iter()
        .map(|row| render_row(row.as_slice(), current, ctx))
        .collect::<Html>()
}

fn render_row(
    row: &[Button<u16>],
    current: u16,
    ctx: &Context<GainSelector>
) -> Html {
    let row_items = row.iter()
        .map(|button| gain_button(current, button.value, button.text.as_str(), ctx))
        .collect::<Html>();

    html!{
        <div>{row_items}</div>
    }
}

fn gain_button(
    current: u16,
    value: u16,
    text: &str,
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
            >{text}
        </button>
    }
}