mod status_bar;
mod footer;

use anyhow::Error;
use ccdi_common::{ClientMessage, StateMessage, ConnectionState, ViewState, LogicStatus};
use yew_websocket::macros::Json;
use gloo::console;
use gloo::timers::callback::Interval;

use base64::{engine::general_purpose::STANDARD, Engine};
use yew::{html, Component, Context, Html, classes};
use yew_websocket::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

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
            ClientMessage::ClientTestResponse(_) => todo!(),
            ClientMessage::View(view) => self.view_state = Some(view),
            ClientMessage::JpegImage(image) => self.jpeg_image = Some(image),
        }

        true
    }

    fn get_logic_status(&self) -> LogicStatus {
        self.view_state.as_ref().map(|state| state.status).unwrap_or(Default::default())
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
            _interval: interval
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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

                    let task = WebSocketService::connect(
                        &ws_url,
                        callback,
                        notification,
                    )
                    .unwrap();
                    self.ws = Some(task);
                    self.connection = ConnectionState::Connecting;
                    true
                }
                WsAction::SendData(message) => {
                    let json = serde_json::to_string(&message).unwrap();
                    self.ws.as_mut().unwrap().send(json);
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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div>
                    <StatusBar connection={self.connection} logic={self.get_logic_status()}/>
                    <nav class="menu">
                        { self.image_data() }
                    </nav>
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