use anyhow::Error;
use ccdi_common::{ClientMessage, StateMessage};
use yew_websocket::macros::Json;
use gloo::console;
use gloo::timers::callback::Interval;

use yew::{html, Component, Context, Html, classes};
use yew_websocket::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

pub enum ConnectedState {
    Disconnected,
    Connecting,
    Established
}

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
    pub data: String,
    pub ws: Option<WebSocketTask>,
    pub connected: ConnectedState,
    _interval: Interval,
}

impl Model {
    fn view_data(&self) -> Html {
        let data_label = match self.data.is_empty() {
            true => "Data hasn't fetched yet.",
            false => self.data.as_str(),
        };

        let (status_label, status_class) = match self.connected {
            ConnectedState::Disconnected => ("Disconnected", "error"),
            ConnectedState::Connecting => ("Connecting ...", "warn"),
            ConnectedState::Established => ("Connected", "ok"),
        };

        html! {
            <div>
            <ul>
                <li class={classes!("status", status_class)}>{status_label}</li>
            </ul>
                <p>{ data_label }</p>
            </div>
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
            data: String::new(),
            ws: None,
            connected: ConnectedState::Disconnected,
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
                    self.connected = ConnectedState::Connecting;
                    true
                }
                WsAction::SendData(message) => {
                    let json = serde_json::to_string(&message).unwrap();
                    self.ws.as_mut().unwrap().send(json);
                    false
                }
                WsAction::Disconnect => {
                    self.ws.take();
                    self.connected = ConnectedState::Disconnected;
                    true
                }
                WsAction::Established => {
                    ctx.link().send_message(WsAction::SendData(StateMessage::ClientConnected));
                    self.connected = ConnectedState::Established;
                    false
                }
                WsAction::Lost => {
                    self.ws = None;
                    true
                }
            },
            Msg::WsReady(response) => {
                self.data = response.map(|data| format!("{:?}", data))
                    .unwrap_or(String::default());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <nav class="menu">
                    { self.view_data() }
                    <button disabled={self.ws.is_some()}
                            onclick={ctx.link().callback(|_| WsAction::Connect)}>
                        { "Connect To WebSocket" }
                    </button>
                    <button disabled={self.ws.is_none()}
                            onclick={ctx.link()
                                .callback(|_| WsAction::SendData(StateMessage::ClientTest(321)))}>
                        { "Send To WebSocket" }
                    </button>
                    <button disabled={self.ws.is_none()}
                            onclick={ctx.link().callback(|_| WsAction::Disconnect)}>
                        { "Close WebSocket connection" }
                    </button>
                </nav>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}