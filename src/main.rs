mod RobotSweetGraph;

use yew::prelude::*;
use RobotSweetGraph::RobotSweetGraphComp;

struct Main;
impl Component for Main {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        html! {
            <RobotSweetGraphComp/>
        }
    }
}

fn main() {
    yew::start_app::<Main>();
}
