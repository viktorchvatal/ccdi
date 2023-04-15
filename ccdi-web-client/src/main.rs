use anyhow::Error;
use ccdi_common::{ClientMessage, StateMessage};
use yew_websocket::macros::Json;
use gloo_timers::callback::Interval;
use gloo::console;

use yew::{html, Component, Context, Html};
use yew_websocket::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

pub enum WsAction {
    Connect,
    SendData,
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
    _interval: Interval,
}

impl Model {
    fn view_data(&self) -> Html {
        let data_label = match self.data.is_empty() {
            true => "Data hasn't fetched yet.",
            false => self.data.as_str(),
        };

        let status_label = match self.ws {
            Some(_) => "Connected",
            None => "Disconnected"
        };

        html! {
            <div>
                <p>{ status_label }</p>
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
        let interval = Interval::new(200, move || callback.emit(()));

        Self {
            fetching: false,
            data: String::new(),
            ws: None,
            _interval: interval
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                if self.ws.is_none() {
                    console::log!("Emitting WsAction::Connect");
                    ctx.link().callback(|_| WsAction::Connect).emit(());
                }
                false
            },
            Msg::WsAction(action) => match action {
                WsAction::Connect => {
                    let callback = ctx.link().callback(|Json(data)| Msg::WsReady(data));
                    let notification = ctx.link().batch_callback(|status| match status {
                        WebSocketStatus::Opened => None,
                        WebSocketStatus::Closed | WebSocketStatus::Error => {
                            Some(WsAction::Lost.into())
                        }
                    });

                    let task = WebSocketService::connect(
                        "ws://127.0.0.1:8081/ccdi",
                        callback,
                        notification,
                    )
                    .unwrap();
                    self.ws = Some(task);
                    true
                }
                WsAction::SendData => {
                    let request = StateMessage::ClientTest(321);
                    let json = serde_json::to_string(&request).unwrap();
                    self.ws.as_mut().unwrap().send(json);
                    false
                }
                WsAction::Disconnect => {
                    self.ws.take();
                    true
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
                            onclick={ctx.link().callback(|_| WsAction::SendData)}>
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