use std::sync::Arc;

use base64::{engine::general_purpose::STANDARD, Engine};
use ccdi_common::RgbImage;
use ccdi_image::{Transform, TransformFunction, rgb_image_to_jpeg, compute_image_stats, ImageStats};
use yew::Properties;
use super::*;

// ============================================ PUBLIC =============================================

pub enum Msg {
    ChangeGain(i32),
    ChangeFunction(TransformFunction),
}

pub struct Picture {
    gain: i32,
    function: TransformFunction,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PictureData {
    pub image: Option<Arc<RgbImage<u16>>>,
}

impl Component for Picture {
    type Message = Msg;
    type Properties = PictureData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            gain: 1,
            function: TransformFunction::Sqrt,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeGain(value) => self.gain = value,
            Msg::ChangeFunction(function) => self.function = function,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let transform = Transform {
            gain: self.gain,
            function: self.function,
            sub: 500
        };

        html! {
            <div>
                <div class="image-main">
                    <div  class="image-tools">
                        <p>{"View"}</p>
                        <hr />
                        <p>{"Gain"}</p>
                        { gain_button(ctx, self.gain, 1) }
                        { gain_button(ctx, self.gain, 2) }
                        { gain_button(ctx, self.gain, 4) }
                        { gain_button(ctx, self.gain, 8) }
                        { gain_button(ctx, self.gain, 16) }
                        { gain_button(ctx, self.gain, 32) }
                        { gain_button(ctx, self.gain, 64) }
                        <p>{"Func"}</p>
                        { function_button(ctx, self.function, TransformFunction::Linear, "Line") }
                        { function_button(ctx, self.function, TransformFunction::Sqrt, "Sqrt") }
                        { function_button(ctx, self.function, TransformFunction::Log2, "Log2") }
                    </div>
                    <div class="image-content">
                        {rgb_image_to_html(ctx.props().image.as_deref(), transform)}
                    </div>
                </div>
                <div>
                    {render_stats(ctx.props().image.as_deref().map(|img| compute_image_stats(img)))
                    }
                </div>
            </div>
        }
    }
}

fn function_button(
    ctx: &Context<Picture>,
    current_function: TransformFunction,
    button_function: TransformFunction,
    text: &str,
) -> Html {
    let function_click = |value: TransformFunction| ctx.link()
        .callback(move |_| Msg::ChangeFunction(value));

    let selected_class = match current_function == button_function {
        true => Some("button-selected"),
        false => None,
    };

    html!{
        <button
            class={classes!("short-button", selected_class)}
            onclick={function_click(button_function)}
        >{ text }</button>
    }
}

fn gain_button(ctx: &Context<Picture>, current_gain: i32, button_gain: i32) -> Html {
    let gain_click = |value: i32| ctx.link().callback(move |_| Msg::ChangeGain(value));

    let selected_class = match current_gain == button_gain {
        true => Some("button-selected"),
        false => None,
    };

    html!{
        <button
            class={classes!("short-button", selected_class)}
            onclick={gain_click(button_gain)}
        >{
            format!("X {}", button_gain)
        }</button>
    }
}

fn rgb_image_to_html(image: Option<&RgbImage<u16>>, transform: Transform) -> Html {
    match image.and_then(|image| rgb_to_jpeg_base64(image, transform)) {
        None => html! { },
        Some(ref base64) => html! {
            <img src={format!("data:image/jpeg;base64,{}", base64)} />
        }
    }
}

fn rgb_to_jpeg_base64(image: &RgbImage<u16>, transform: Transform) -> Option<String> {
    let encoded_jpeg = rgb_image_to_jpeg(image, transform).ok()?;
    let encoded_base64 = STANDARD.encode(&encoded_jpeg);
    Some(encoded_base64)
}

fn render_stats(stats: Option<ImageStats>) -> Html {
    let stats_text = match stats {
        None => String::new(),
        Some(stats) => format!(
            "All: [{}–{}] R: [{}–{}] G: [{}–{}] B: [{}–{}]",
            stats.total.min, stats.total.max,
            stats.r.min, stats.r.max,
            stats.g.min, stats.g.max,
            stats.b.min, stats.b.max,
        )
    };

    html!{
        <div>{stats_text}</div>
    }
}