use yew::{Properties, Callback};
use super::*;

// ============================================ PUBLIC =============================================


/// The `Child` component is the child of the `Parent` component, and will receive updates from the
/// parent using properties.
pub struct Menu;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MenuItem {
    Composition,
    Camera
}

pub enum Msg {
    Click(MenuItem),
}

#[derive(Clone, PartialEq, Properties)]
pub struct MenuData {
    pub clicked: Callback<MenuItem>,
}

impl Component for Menu {
    type Message = Msg;
    type Properties = MenuData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click(action) => ctx.props().clicked.emit(action),
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        use MenuItem::*;
        let menu_click = |action: MenuItem| ctx.link().callback(move |_| Msg::Click(action));

        html! {
            <div>
                <button onclick={menu_click(Composition)}>{"Composition"}</button>
                <button onclick={menu_click(Camera)}>{"Camera"}</button>
            </div>
        }
    }
}
