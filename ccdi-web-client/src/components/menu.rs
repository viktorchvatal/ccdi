use yew::{Properties, Callback};
use super::*;

// ============================================ PUBLIC =============================================


/// The `Child` component is the child of the `Parent` component, and will receive updates from the
/// parent using properties.
pub struct Menu;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MenuItem {
    Composition,
    Cooling,
    Info,
    Shoot,
}

pub enum Msg {
    Click(MenuItem),
}

#[derive(Clone, PartialEq, Properties)]
pub struct MenuData {
    pub clicked: Callback<MenuItem>,
    pub selected: MenuItem
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
        let selected = ctx.props().selected;

        html! {
            <div>
                {menu_item("Composition", Composition, selected, ctx)}
                {menu_item("Cooling", Cooling, selected, ctx)}
                {menu_item("Info", Info, selected, ctx)}
                {menu_item("Series", Shoot, selected, ctx)}
            </div>
        }
    }
}

// =========================================== PRIVATE =============================================

fn menu_item(
    name: &'static str,
    item: MenuItem,
    selected: MenuItem,
    ctx: &Context<Menu>
) -> Html {
    let menu_click = |action: MenuItem| ctx.link().callback(move |_| Msg::Click(action));

    let selected_class = match item == selected {
        true => Some("button-selected"),
        false => None,
    };

    html! {
        <button class={classes!(selected_class)} onclick={menu_click(item)}>{name}</button>
    }
}