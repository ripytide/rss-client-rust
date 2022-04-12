use robot_sweet_shop::state2::RobotSweetGraph;
use yew::prelude::*;

struct RobotSweetStateDrawParams {
    image_scale: f32,
}

#[function_component(RobotSweetGraphComp)]
pub fn robot_sweet_state() -> Html {
    let state = use_state(|| fetch_state());
}
