mod status_bar;
mod footer;
mod menu;
mod camera;
mod composition;

use anyhow::Error;
use ccdi_common::{ClientMessage, StateMessage, ConnectionState, ViewState, LogicStatus};
use ccdi_image::simple_raw_image_to_jpeg;
use composition::CompositionDetail;
use yew_websocket::macros::Json;
use gloo::console;
use gloo::timers::callback::Interval;

use base64::{engine::general_purpose::STANDARD, Engine};
use yew::{html, Component, Context, Html, classes};
use yew_websocket::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

use crate::camera::CameraDetail;
use crate::menu::{Menu, MenuItem};
use crate::status_bar::StatusBar;
use crate::footer::Footer;

// ============================================ PUBLIC =============================================

pub enum WsAction {
    Connect,
    SendData(StateMessage),
    Established,
    Disconnect,
    Lost,
}

pub enum Msg {
    Tick,
    WsAction(WsAction),
    WsReady(Result<ClientMessage, Error>),
    Action(UserAction)
}

pub enum UserAction {
    MenuClick(MenuItem),
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

pub struct Model {
    pub fetching: bool,
    pub jpeg_image: Option<Vec<u8>>,
    pub ws: Option<WebSocketTask>,
    pub connection: ConnectionState,
    pub view_state: Option<ViewState>,
    pub selected_menu: MenuItem,
    _interval: Interval,
}

impl Model {
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
            .callback(|action: StateMessage| Msg::WsAction(WsAction::SendData(action)));

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

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(600, move || callback.emit(()));

        Self {
            fetching: false,
            ws: None,
            jpeg_image: None,
            connection: ConnectionState::Disconnected,
            view_state: None,
            selected_menu: MenuItem::Camera,
            _interval: interval
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Action(action) => {
                match action {
                    UserAction::MenuClick(menuitem) => {
                        self.selected_menu = menuitem;
                        true
                    },
                }
            }
            Msg::Tick => {
                if self.ws.is_none() {
                    ctx.link().send_message(WsAction::Connect);
                }
                false
            },
            Msg::WsAction(action) => match action {
                WsAction::Connect => {
                    let hostname = gloo::utils::window().location().hostname().ok()
                        .unwrap_or(String::from("localhost"));

                    let ws_url = format!("ws://{}:8081/ccdi", hostname);

                    console::info!(&hostname, "WS: ", &ws_url);
                    let callback = ctx.link().callback(|Json(data)| Msg::WsReady(data));
                    let notification = ctx.link().batch_callback(|status| match status {
                        WebSocketStatus::Opened => Some(WsAction::Established.into()),
                        WebSocketStatus::Closed | WebSocketStatus::Error => {
                            Some(WsAction::Lost.into())
                        }
                    });

                    if let Ok(connection) = WebSocketService::connect(
                        &ws_url,
                        callback,
                        notification,
                    ) {
                        self.ws = Some(connection);
                        self.connection = ConnectionState::Connecting;
                    } else {
                        console::error!("Failed to create web socket service");
                    }
                    true
                }
                WsAction::SendData(message) => {
                    if let Ok(json) = serde_json::to_string(&message) {
                        if let Some(ref mut ws) = self.ws {
                            ws.send(json)
                        }
                    }
                    false
                }
                WsAction::Disconnect => {
                    self.ws.take();
                    self.connection = ConnectionState::Disconnected;
                    true
                }
                WsAction::Established => {
                    ctx.link().send_message(WsAction::SendData(StateMessage::ClientConnected));
                    self.connection = ConnectionState::Established;
                    false
                }
                WsAction::Lost => {
                    self.ws = None;
                    true
                }
            },
            Msg::WsReady(response) => {
                if let Ok(message) = response {
                    self.receive_message(message)
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let menu_clicked = ctx.link()
            .callback(|action: MenuItem| Msg::Action(UserAction::MenuClick(action)));

        html! {
            <>
                <StatusBar connection={self.connection} logic={self.get_logic_status()}/>
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
    yew::Renderer::<Model>::new().render();
}