use yew::{Properties, Callback};
use super::*;

// ============================================ PUBLIC =============================================

pub struct CoolingSelector;

pub enum Msg {
    SetTemp(f32),
}

#[derive(Clone, PartialEq, Properties)]
pub struct TimeData {
    pub temp_changed: Callback<f32>,
    pub selected_temp: f32,
    pub config: ButtonSet<f32>,
}

impl Component for CoolingSelector {
    type Message = Msg;
    type Properties = TimeData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetTemp(value) => {
                ctx.props().temp_changed.emit(value)
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let selected = ctx.props().selected_temp;
        let buttons = &ctx.props().config;

        html! {
            <div>
                <p>{"Set camera temperature"}</p>
                {render_buttons(buttons, selected, ctx)}
            </div>
        }
    }
}

// =========================================== PRIVATE =============================================

fn render_buttons(
    button_set: &ButtonSet<f32>,
    current: f32,
    ctx: &Context<CoolingSelector>
) -> Html {
    button_set.buttons.iter()
        .map(|row| render_row(row.as_slice(), current, ctx))
        .collect::<Html>()
}

fn render_row(
    row: &[Button<f32>],
    current: f32,
    ctx: &Context<CoolingSelector>
) -> Html {
    let row_items = row.iter()
        .map(|button| cooling_button(current, button.value, button.text.as_str(), ctx))
        .collect::<Html>();

    html!{
        <div>{row_items}</div>
    }
}

fn cooling_button(
    current: f32,
    value: f32,
    text: &str,
    ctx: &Context<CoolingSelector>
) -> Html {
    let time_click = |action: f32| ctx.link().callback(move |_| Msg::SetTemp(action));

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

