use yew::{Properties, Callback};
use super::*;

// ============================================ PUBLIC =============================================

pub struct System {
    confirmation_enabled: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SystemData {
    pub on_action: Callback<StateMessage>,
}

pub enum Msg{
    ServerAction(StateMessage),
    ShowConfirmation,
}

impl Component for System {
    type Message = Msg;
    type Properties = SystemData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            confirmation_enabled: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ServerAction(action) => ctx.props().on_action.emit(action),
            Msg::ShowConfirmation => self.confirmation_enabled = !self.confirmation_enabled,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        use StateMessage::*;

        let server_action = |action: StateMessage| ctx.link().callback(
            move |_| Msg::ServerAction(action.clone())
        );

        let show_confirm = || ctx.link().callback(move |_| Msg::ShowConfirmation);

        let confirmation_button = match self.confirmation_enabled {
            false => html!{},
            true => html!{
                <button onclick={server_action(PowerOff)}>{"Power Off !"}</button>
            }
        };

        html!{
            <div>
                <p>{"Commands"}</p>
                <button onclick={show_confirm()}>{"Power Off ?"}</button>
                {confirmation_button}
                <br/>
                <p>{"2025-07-27"}</p>
            </div>
        }
    }
}
