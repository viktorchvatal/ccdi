use yew::{Properties, Callback};
use super::*;

// ============================================ PUBLIC =============================================

pub struct TimeSelector;

pub enum Msg {
    SetTime(f64),
}

#[derive(Clone, PartialEq, Properties)]
pub struct TimeData {
    pub time_changed: Callback<f64>,
    pub selected_time: f64,
    pub config: ButtonSet<f64>,
}

impl Component for TimeSelector {
    type Message = Msg;
    type Properties = TimeData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetTime(value) => {
                ctx.props().time_changed.emit(value)
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let selected = ctx.props().selected_time;
        let buttons = &ctx.props().config;

        html! {
            <div>
                <p>{"Set camera exposure time"}</p>
                {render_buttons(buttons, selected, ctx)}
            </div>
        }
    }
}

// =========================================== PRIVATE =============================================

fn render_buttons(
    button_set: &ButtonSet<f64>,
    current: f64,
    ctx: &Context<TimeSelector>
) -> Html {
    button_set.buttons.iter()
        .map(|row| render_row(row.as_slice(), current, ctx))
        .collect::<Html>()
}

fn render_row(
    row: &[Button<f64>],
    current: f64,
    ctx: &Context<TimeSelector>
) -> Html {
    let row_items = row.iter()
        .map(|button| time_button(current, button.value, button.text.as_str(), ctx))
        .collect::<Html>();

    html!{
        <div>{row_items}</div>
    }
}

fn time_button(
    current: f64,
    value: f64,
    text: &str,
    ctx: &Context<TimeSelector>
) -> Html {
    let time_click = |action: f64| ctx.link().callback(move |_| Msg::SetTime(action));

    let selected_class = match value == current {
        true => Some("button-selected"),
        false => None,
    };

    html! {
        <button
            class={classes!("short-button", selected_class)}
            onclick={time_click(value)}
            >{text}
        </button>
    }
}
