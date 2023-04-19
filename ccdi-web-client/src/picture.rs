use ccdi_common::RgbImage;
use yew::Properties;
use super::*;

// ============================================ PUBLIC =============================================

pub struct Picture {

}

#[derive(Clone, PartialEq, Properties)]
pub struct PictureData {
    pub image: Option<Rc<RgbImage<u16>>>,
}

impl Component for Picture {
    type Message = ();
    type Properties = PictureData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {

        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                {rgb_image_to_html(ctx.props().image.as_deref())}
            </div>
        }
    }
}

fn rgb_image_to_html(image: Option<&RgbImage<u16>>) -> Html {
    match image.and_then(|image| rgb_to_jpeg_base64(image)) {
        None => html! { },
        Some(ref base64) => html! {
            <img src={format!("data:image/jpeg;base64,{}", base64)} />
        }
    }
}

fn rgb_to_jpeg_base64(image: &RgbImage<u16>) -> Option<String> {
    let encoded_jpeg = rgb_image_to_jpeg(image).ok()?;
    let encoded_base64 = STANDARD.encode(&encoded_jpeg);
    Some(encoded_base64)
}