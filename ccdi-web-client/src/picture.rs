use ccdi_common::RgbImage;
use ccdi_image::Transform;
use yew::Properties;
use super::*;

// ============================================ PUBLIC =============================================

pub enum Msg {
    ChangeGain(i32)
}

pub struct Picture {
    gain: i32,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PictureData {
    pub image: Option<Rc<RgbImage<u16>>>,
}

impl Component for Picture {
    type Message = Msg;
    type Properties = PictureData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            gain: 1,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeGain(value) => self.gain = value,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let gain_click = |value: i32| ctx.link().callback(move |_| Msg::ChangeGain(value));

        let transform = Transform {
            gain: self.gain,
            sub: 500
        };

        html! {
            <div>
                <div>
                    <button onclick={gain_click(1)}>{"x 1"}</button>
                    <button onclick={gain_click(4)}>{"x 4"}</button>
                    <button onclick={gain_click(16)}>{"x 16"}</button>
                    <button onclick={gain_click(64)}>{"x 64"}</button>
                </div>
                <div>
                    {rgb_image_to_html(ctx.props().image.as_deref(), transform)}
                </div>
            </div>
        }
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