use ccdi_common::ExposureCommand;
use wasm_bindgen::{UnwrapThrowExt, JsCast};
use web_sys::{HtmlInputElement, Event};
use yew::{Properties, Callback, use_state, InputEvent};
use crate::components::text_input::TextInput;

use super::*;

// ============================================ PUBLIC =============================================

pub struct ShootingDetail {
    pub edited_name: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ShootingDetailData {
    pub on_action: Callback<StateMessage>,
}

pub enum Msg{
    UpdateEditedName(String),
    SetDirectory,
}

impl Component for ShootingDetail {
    type Message = Msg;
    type Properties = ShootingDetailData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            edited_name: String::new()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateEditedName(name) => self.edited_name = name,
            Msg::SetDirectory => ctx.props().on_action.emit(
                StateMessage::StorageMessage(
                    StorageMessage::SetDirectory(self.edited_name.clone())
                )
            ),
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(Msg::UpdateEditedName);
        let set_dir_click = || ctx.link().callback(move |_| Msg::SetDirectory);

        html!{
            <div>
                <div>
                    <p>{"Directory"}</p>
                    <TextInput {on_change} value={self.edited_name.clone()}/>
                    <div>{self.edited_name.as_str()}</div>
                    <button onclick={set_dir_click()}>{"Set dir"}
                    </button>
                </div>
            </div>
        }
    }
}
