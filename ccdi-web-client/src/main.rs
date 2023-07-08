mod components;
mod connection;
mod selectors;

use std::sync::Arc;

use ccdi_common::*;
use connection::ConnectionService;
use gloo::console;

use yew::html::Scope;
use yew::{html, Component, Context, Html};

use components::composition::CompositionDetail;
use components::camera::CameraDetail;
use components::footer::Footer;
use components::menu::{Menu, MenuItem};
use components::status_bar::StatusBar;
use selectors::picture::Picture;
use selectors::time::TimeSelector;
use selectors::gain::GainSelector;
use selectors::cooling::CoolingSelector;
use selectors::rendering::RenderingSelector;

// ============================================ PUBLIC =============================================

pub enum Msg {
    RegisterConnectionService(Scope<ConnectionService>),
    ConnectionState(ConnectionState),
    MessageReceived(ClientMessage),
    SendMessage(StateMessage),
    Action(UserAction),
    ParamUpdate(CameraParamMessage),
}

pub enum UserAction {
    MenuClick(MenuItem),
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
            ClientMessage::Reconnect => {} // handled elsewhere
            ClientMessage::View(view) => self.view_state = view,
            ClientMessage::RgbImage(image) => self.image = Some(image),
        }

        true
    }

    fn render_tool(&self, ctx: &Context<Self>) -> Html {
        match self.selected_menu {
            MenuItem::Composition => self.render_composition(ctx),
            MenuItem::Cooling => self.render_cooling(ctx),
            MenuItem::Shoot => self.render_shoot(ctx),
            MenuItem::Info => html!{
                <CameraDetail data={self.view_state.camera_properties.clone()} />
            },
        }
    }

    fn render_composition(&self, ctx: &Context<Self>) -> Html {
        let action = ctx.link()
            .callback(|action: StateMessage| Msg::SendMessage(action));

        let gain_changed = ctx.link().callback(
            |gain: u16| Msg::ParamUpdate(CameraParamMessage::SetGain(gain))
        );

        let time_changed = ctx.link().callback(
            |time: f64| Msg::ParamUpdate(CameraParamMessage::SetTime(time))
        );

        let rendering_changed = ctx.link().callback(
            |value: RenderingType| Msg::ParamUpdate(CameraParamMessage::SetRenderingType(value))
        );

        html!{
            <div>
                <GainSelector
                    config={self.view_state.config.gain.clone()}
                    gain_changed={gain_changed}
                    selected_gain={self.view_state.camera_params.gain}
                />
                <TimeSelector
                    config={self.view_state.config.exposure.clone()}
                    time_changed={time_changed}
                    selected_time={self.view_state.camera_params.time}
                />
                <RenderingSelector
                    rendering_changed={rendering_changed}
                    selected_value={self.view_state.camera_params.rendering}
                />
                <CompositionDetail
                    on_action={action}
                    camera_params={self.view_state.camera_params.clone()}
                />
            </div>
        }
    }

    fn render_cooling(&self, ctx: &Context<Self>) -> Html {
        let temperature_changed = ctx.link().callback(
            |temp: f32| Msg::ParamUpdate(CameraParamMessage::SetTemp(temp))
        );

        html!{
            <CoolingSelector
                config={self.view_state.config.temperature.clone()}
                selected_temp={self.view_state.camera_params.temperature}
                temp_changed={temperature_changed}
            />
        }
    }

    fn render_shoot(&self, ctx: &Context<Self>) -> Html {
        let action = ctx.link()
            .callback(|action: StateMessage| Msg::SendMessage(action));

        html!{
            <CompositionDetail
                on_action={action}
                camera_params={self.view_state.camera_params.clone()}
            />
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
            },
            Msg::ParamUpdate(message) => {
                ctx.link().send_message(Msg::SendMessage(StateMessage::CameraParam(message)));
                false
            }
            Msg::Action(action) => {
                match action {
                    UserAction::MenuClick(menuitem) => {
                        self.selected_menu = menuitem;
                    },
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

        html! {
            <>
                <ConnectionService
                    on_message={client_message_received}
                    on_state_change={connection_state_changed}
                />
                <StatusBar
                    connection={self.connection_state}
                    logic={self.view_state.status.clone()}
                />
                <Menu clicked={menu_clicked} selected={self.selected_menu} />
                <div class="main-row">
                    <div class="main-image-column">
                        <Picture image={self.image.clone()} />
                    </div>
                    <div class="main-tool-column">
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