mod status_bar;
mod footer;
mod menu;
mod camera;
mod composition;
mod connection;

use ccdi_common::{ClientMessage, StateMessage, ConnectionState, ViewState, LogicStatus};
use ccdi_image::simple_raw_image_to_jpeg;
use composition::CompositionDetail;
use connection::{ConnectionService};
use gloo::console;

use base64::{engine::general_purpose::STANDARD, Engine};
use yew::html::Scope;
use yew::{html, Component, Context, Html, classes};

use crate::camera::CameraDetail;
use crate::menu::{Menu, MenuItem};
use crate::status_bar::StatusBar;
use crate::footer::Footer;

// ============================================ PUBLIC =============================================

pub enum Msg {
    RegisterConnectionService(Scope<ConnectionService>),
    ConnectionState(ConnectionState),
    MessageReceived(ClientMessage),
    MessageSent(StateMessage),
    Action(UserAction)
}

pub enum UserAction {
    MenuClick(MenuItem),
}

pub struct Main {
    pub jpeg_image: Option<Vec<u8>>,
    pub view_state: Option<ViewState>,
    pub selected_menu: MenuItem,
    pub connection_state: ConnectionState,
    pub conection_context: Option<Scope<ConnectionService>>,
}

impl Main {
    fn image_data(&self) -> Html {
        match self.jpeg_image {
            None => html! { },
            Some(ref data) => html! {
                <img src={format!("data:image/jpeg;base64,{}", STANDARD.encode(&data))} />
            }
        }
    }

    fn receive_message(&mut self, message: ClientMessage) -> bool {
        match message {
            ClientMessage::View(view) => self.view_state = Some(view),
            ClientMessage::JpegImage(image) => self.jpeg_image = Some(image),
            ClientMessage::RawImage(image) => {
                let (w, h) = (image.params.area.width, image.params.area.height);
                let mpix = w*h/1024/1024;
                console::info!(&format!("Acquired {} x {} image with {} MPixels", w, h, mpix));

                match simple_raw_image_to_jpeg(&image, 10) {
                    Ok(jpeg) => self.jpeg_image = Some(jpeg),
                    Err(error) => console::info!(&format!("Jpeg convert failed {}", error)),
                }
            }
        }

        true
    }

    fn get_logic_status(&self) -> LogicStatus {
        self.view_state.as_ref().map(|state| state.status).unwrap_or(Default::default())
    }

    fn view_part<T, F: Fn(&ViewState) -> Option<T>>(&self, mapper: F) -> Option<T> {
        self.view_state.as_ref().and_then(|state| mapper(state))
    }

    fn render_tool(&self, ctx: &Context<Self>) -> Html {
        let action = ctx.link()
            .callback(|action: StateMessage| Msg::MessageSent(action));

        match self.selected_menu {
            MenuItem::Composition => html!{
                <CompositionDetail on_action={action} />
            },
            MenuItem::Camera => html!{
                <CameraDetail data={self.view_part(|state| state.camera_properties.clone())} />
            },
        }
    }
}

impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            jpeg_image: None,
            view_state: None,
            selected_menu: MenuItem::Camera,
            connection_state: ConnectionState::Disconnected,
            conection_context: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MessageSent(message) => {
                match self.conection_context.as_ref() {
                    None => console::warn!("No connection service registered."),
                    Some(context) => context.send_message(connection::Msg::SendData(message)),
                }
                false
            }
            Msg::RegisterConnectionService(context) => {
                self.conection_context = Some(context);
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
                        true
                    },
                }
            }
            Msg::MessageReceived(message) => {
                self.receive_message(message)
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let menu_clicked = ctx.link()
            .callback(|action: MenuItem| Msg::Action(UserAction::MenuClick(action)));

        let client_message_received = ctx.link()
            .callback(|message: ClientMessage| Msg::MessageReceived(message));

        let connection_state_changed = ctx.link()
            .callback(|state: ConnectionState| Msg::ConnectionState(state));

        html! {
            <>
                <ConnectionService
                    on_message={client_message_received}
                    on_state_change={connection_state_changed}
                />
                <StatusBar connection={self.connection_state} logic={self.get_logic_status()}/>
                <Menu clicked={menu_clicked} selected={self.selected_menu} />
                <div class="main-row">
                    <div class="main-image-column">
                        { self.image_data() }
                    </div>
                    <div class="main-tool-column">
                        { self.render_tool(ctx) }
                    </div>
                </div>
                <Footer text={
                    self.view_state.as_ref()
                        .map(|view| view.detail.clone())
                        .unwrap_or(String::new())}
                />
            </>
        }
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}