use yew::{Properties, Callback};
use super::*;

// ============================================ PUBLIC =============================================

pub struct RenderingSelector;

pub enum Msg {
    SetValue(RenderingType),
}

#[derive(Clone, PartialEq, Properties)]
pub struct RenderingData {
    pub rendering_changed: Callback<RenderingType>,
    pub selected_value: RenderingType,
}

impl Component for RenderingSelector {
    type Message = Msg;
    type Properties = RenderingData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetValue(value) => {
                ctx.props().rendering_changed.emit(value)
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let selected = ctx.props().selected_value;

        html! {
            <div>
                <p>{"View transformation"}</p>
                { rendering_button(selected, RenderingType::FullImage, "Full Image", ctx)}
                { rendering_button(selected, RenderingType::Center1x, "Center 1:1", ctx)}
                { rendering_button(selected, RenderingType::Corners1x, "Corners 1:1", ctx)}
            </div>
        }
    }
}

// =========================================== PRIVATE =============================================

fn rendering_button(
    current: RenderingType,
    value: RenderingType,
    text: &str,
    ctx: &Context<RenderingSelector>
) -> Html {
    let gain_click = |action: RenderingType| ctx.link().callback(move |_| Msg::SetValue(action));

    let selected_class = match value == current {
        true => Some("button-selected"),
        false => None,
    };

    html! {
        <button class={classes!(selected_class)} onclick={gain_click(value)}>{text}</button>
    }
}