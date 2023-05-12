use anyhow::Error;
use yew::{html, Component, Context, Html, Callback, Properties};
use yew_websocket::{websocket::{WebSocketService, WebSocketStatus, WebSocketTask}, macros::Json};
use ccdi_common::{ClientMessage, StateMessage, ConnectionState};
use gloo::console;
use gloo::timers::callback::Interval;

// ============================================ PUBLIC =============================================

pub struct ConnectionService {
    ws: Option<WebSocketTask>,
    _interval: Interval,
}

pub enum Msg {
    Tick,
    Connect,
    SendData(StateMessage),
    DataReceived(Result<ClientMessage, Error>),
    Established,
    Disconnect,
    Lost,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ConnectionProperties {
    pub on_message: Callback<ClientMessage>,
    pub on_state_change: Callback<ConnectionState>,
}

impl Component for ConnectionService {
    type Message = Msg;
    type Properties = ConnectionProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(600, move || callback.emit(()));

        Self {
            ws: None,
            _interval: interval
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let child_link = ctx.link().clone();
        let parent_link = ctx.link().get_parent().expect("No Parent found").clone();
        parent_link.downcast::<super::Main>().send_message(
            super::Msg::RegisterConnectionService(child_link)
        );

        match msg {
            Msg::Tick => {
                if self.ws.is_none() {
                    ctx.link().send_message(Msg::Connect);
                }
                false
            },
            Msg::Connect => {
                let hostname = gloo::utils::window().location().hostname().ok()
                    .unwrap_or(String::from("localhost"));

                let ws_url = format!("ws://{}:8081/ccdi", hostname);

                console::info!(&hostname, "WS: ", &ws_url);
                let callback = ctx.link().callback(|Json(data)| Msg::DataReceived(data));

                let notification = ctx.link().batch_callback(|status| match status {
                    WebSocketStatus::Opened => Some(Msg::Established),
                    WebSocketStatus::Closed | WebSocketStatus::Error => {
                        Some(Msg::Lost)
                    }
                });

                if let Ok(connection) = WebSocketService::connect(
                    &ws_url,
                    callback,
                    notification,
                ) {
                    self.ws = Some(connection);
                    ctx.props().on_state_change.emit(ConnectionState::Connecting)
                } else {
                    console::error!("Failed to create web socket service");
                }
                true
            }
            Msg::SendData(message) => {
                if let Ok(json) = serde_json::to_string(&message) {
                    if let Some(ref mut ws) = self.ws {
                        ws.send(json)
                    }
                }
                false
            }
            Msg::DataReceived(reception_result) => {
                if let Ok(client_message) = reception_result {
                    if client_message == ClientMessage::Reconnect {
                        // Our server queue got overwhelmed (client got too slow or just did
                        // not receive messages, but websocket was still alive).
                        // Server instructs us to close the websocket and open a new connection
                        // as more messages may not be sent to prevent full memory
                        ctx.link().send_message(Msg::Disconnect)
                    } else {
                        ctx.props().on_message.emit(client_message)
                    }
                }

                false
            }
            Msg::Disconnect => {
                self.ws.take();
                ctx.props().on_state_change.emit(ConnectionState::Disconnected);
                true
            }
            Msg::Established => {
                ctx.link().send_message(Msg::SendData(StateMessage::ClientConnected));
                ctx.props().on_state_change.emit(ConnectionState::Established);
                true
            }
            Msg::Lost => {
                self.ws = None;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {}
    }
}
