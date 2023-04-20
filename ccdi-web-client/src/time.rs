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

        html! {
            <div>
                <p>{"Set camera exposure time"}</p>
                <div>
                    { time_button(selected, 0.1, ctx)}
                    { time_button(selected, 0.15, ctx)}
                    { time_button(selected, 0.2, ctx)}
                    { time_button(selected, 0.3, ctx)}
                    { time_button(selected, 0.5, ctx)}
                    { time_button(selected, 0.7, ctx)}
                </div>
                <div>
                    { time_button(selected, 1.0, ctx)}
                    { time_button(selected, 1.5, ctx)}
                    { time_button(selected, 2.0, ctx)}
                    { time_button(selected, 3.0, ctx)}
                    { time_button(selected, 5.0, ctx)}
                    { time_button(selected, 7.0, ctx)}
                </div>
                <div>
                    { time_button(selected, 10.0, ctx)}
                    { time_button(selected, 15.0, ctx)}
                    { time_button(selected, 20.0, ctx)}
                    { time_button(selected, 30.0, ctx)}
                    { time_button(selected, 40.0, ctx)}
                    { time_button(selected, 60.0, ctx)}
                </div>
                <div>
                    { time_button(selected, 90.0, ctx)}
                    { time_button(selected, 2.0*60.0, ctx)}
                    { time_button(selected, 3.0*60.0, ctx)}
                    { time_button(selected, 5.0*60.0, ctx)}
                    { time_button(selected, 7.0*60.0, ctx)}
                    { time_button(selected, 10.0*60.0, ctx)}
                </div>
            </div>
        }
    }
}

// =========================================== PRIVATE =============================================

fn time_button(
    current: f64,
    value: f64,
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
            >{format_time_value(value)}
        </button>
    }
}

fn format_time_value(value: f64) -> String {
    if value >= 60.0 {
        let minutes = value/60.0;
        format!("{}m", minutes)
    } else {
        if value < 1.0 {
            let fraction = 1.0/value;
            if fraction >= 10.0 {
                format!("1/{:2.0}s", fraction)
            } else {
                format!("1/{:3.1}s", fraction)
            }
        } else {
            format!("{}s", value)
        }
    }
}