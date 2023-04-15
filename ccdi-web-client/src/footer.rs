use yew::Properties;
use super::*;

// ============================================ PUBLIC =============================================

/// The `Child` component is the child of the `Parent` component, and will receive updates from the
/// parent using properties.
pub struct Footer;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct FooterData {
    pub text: String,
}

impl Component for Footer {
    type Message = ();
    type Properties = FooterData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <footer>
                { ctx.props().text.as_str() }
            </footer>
        }
    }
}
