use yew::Properties;
use super::*;

// ============================================ PUBLIC =============================================

/// The `Child` component is the child of the `Parent` component, and will receive updates from the
/// parent using properties.
pub struct StatusBar;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct StatusBarData {
    pub connection: ConnectionState,
    pub logic: LogicStatus,
}

impl Component for StatusBar {
    type Message = ();
    type Properties = StatusBarData;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="status-bar-body float-container">
                { state_view("Connection", ctx.props().connection) }
                { state_view("Camera", ctx.props().logic.camera) }
            </div>
        }
    }
}

// =========================================== PRIVATE =============================================

fn state_view(name: &str, state: ConnectionState) -> Html {
    let status_class = status_to_class(state);

    html! {
        <ul class="float-child">
            <li class={classes!("status", status_class)}>{name}</li>
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