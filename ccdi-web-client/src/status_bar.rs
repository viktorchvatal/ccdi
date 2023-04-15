use yew::Properties;
use super::*;

// ============================================ PUBLIC =============================================

/// The `Child` component is the child of the `Parent` component, and will receive updates from the
/// parent using properties.
pub struct StatusBar;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Established
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct StatusBarData {
    pub connection: ConnectionState,
}

impl Component for StatusBar {
    type Message = ();
    type Properties = StatusBarData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="status-bar-body">
                { connection_state(ctx.props().connection) }
            </div>
        }
    }
}

// =========================================== PRIVATE =============================================

fn connection_state(state: ConnectionState) -> Html {
    let status_class = status_to_class(state);

    html! {
        <ul>
            <li class={classes!("status", status_class)}>{"Connection"}</li>
        </ul>
    }
}

fn status_to_class(state: ConnectionState) -> &'static str {
    match state {
        ConnectionState::Disconnected => "error",
        ConnectionState::Connecting => "warn",
        ConnectionState::Established => "ok",
    }
}