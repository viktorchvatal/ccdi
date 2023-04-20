mod status_bar;
mod footer;
mod menu;
mod camera;
mod composition;
mod connection;
mod picture;
mod gain;
mod time;

use std::sync::Arc;

use ccdi_common::*;
use ccdi_image::{rgb_image_to_jpeg};
use composition::CompositionDetail;
use connection::{ConnectionService};
use gloo::console;

use base64::{engine::general_purpose::STANDARD, Engine};
use yew::html::Scope;
use yew::{html, Component, Context, Html, classes};

use crate::camera::CameraDetail;
use crate::gain::GainSelector;
use crate::menu::{Menu, MenuItem};
use crate::picture::Picture;
use crate::status_bar::StatusBar;
use crate::footer::Footer;
use crate::time::TimeSelector;

// ============================================ PUBLIC =============================================

pub enum Msg {
    RegisterConnectionService(Scope<ConnectionService>),
    ConnectionState(ConnectionState),
    MessageReceived(ClientMessage),
    SendMessage(StateMessage),
    Action(UserAction),
}

pub enum UserAction {
    MenuClick(MenuItem),
    SetGain(u16),
    SetExposure(f64),
}

pub struct Main {
    pub image: Option<Arc<RgbImage<u16>>>,
    pub view_state: ViewState,
    pub connection_state: ConnectionState,
    pub connection_context: Option<Scope<ConnectionService>>,
    pub selected_menu: MenuItem,
}

impl Main {
    fn receive_message(&mut self, message: ClientMessage) -> bool {
        match message {
            ClientMessage::View(view) => self.view_state = view,
            ClientMessage::RgbImage(image) => self.image = Some(image),
        }

        true
    }

    fn render_tool(&self, ctx: &Context<Self>) -> Html {
        let action = ctx.link()
            .callback(|action: StateMessage| Msg::SendMessage(action));

        match self.selected_menu {
            MenuItem::Composition => html!{
                <CompositionDetail on_action={action} />
            },
            MenuItem::Camera => html!{
                <CameraDetail data={self.view_state.camera_properties.clone()} />
            },
        }
    }
}

impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            image: None,
            view_state: Default::default(),
            selected_menu: MenuItem::Composition,
            connection_state: ConnectionState::Disconnected,
            connection_context: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SendMessage(message) => {
                match self.connection_context.as_ref() {
                    None => console::warn!("No connection service registered."),
                    Some(context) => context.send_message(connection::Msg::SendData(message)),
                }
                false
            }
            Msg::RegisterConnectionService(context) => {
                self.connection_context = Some(context);
                false
            }
            Msg::ConnectionState(state) => {
                self.connection_state = state;
                true
            }
            Msg::Action(action) => {
                match action {
                    UserAction::MenuClick(menuitem) => {
                        self.selected_menu = menuitem;
                    },
                    UserAction::SetExposure(exposure) => {
                        ctx.link().send_message(Msg::SendMessage(
                            StateMessage::ExposureMessage(ExposureCommand::SetTime(exposure))
                        ));
                    },
                    UserAction::SetGain(gain) => {
                        ctx.link().send_message(Msg::SendMessage(
                            StateMessage::ExposureMessage(ExposureCommand::SetGain(gain))
                        ));
                    }
                }
                true
            }
            Msg::MessageReceived(message) => {
                self.receive_message(message)
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let menu_clicked = ctx.link().callback(
            |action: MenuItem| Msg::Action(UserAction::MenuClick(action))
        );

        let client_message_received = ctx.link().callback(
            |message: ClientMessage| Msg::MessageReceived(message)
        );

        let connection_state_changed = ctx.link().callback(
            |state: ConnectionState| Msg::ConnectionState(state)
        );

        let gain_changed = ctx.link().callback(
            |gain: u16| Msg::Action(UserAction::SetGain(gain))
        );

        let time_changed = ctx.link().callback(
            |time: f64| Msg::Action(UserAction::SetExposure(time))
        );

        html! {
            <>
                <ConnectionService
                    on_message={client_message_received}
                    on_state_change={connection_state_changed}
                />
                <StatusBar connection={self.connection_state} logic={self.view_state.status}/>
                <Menu clicked={menu_clicked} selected={self.selected_menu} />
                <div class="main-row">
                    <div class="main-image-column">
                        <Picture image={self.image.clone()} />
                    </div>
                    <div class="main-tool-column">
                        <GainSelector gain_changed={gain_changed} selected_gain={self.view_state.gain} />
                        <TimeSelector time_changed={time_changed} selected_time={self.view_state.time} />
                        { self.render_tool(ctx) }
                    </div>
                </div>
                <Footer text={self.view_state.detail.clone()}
                />
            </>
        }
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}