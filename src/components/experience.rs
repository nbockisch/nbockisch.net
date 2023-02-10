//! Contains the `Experience` component with the professional experience I've
//! had
use stylist::css;
use yew::{
    classes,
    function_component,
    Html,
    html,
};

/// Component that displays my professional experience
///
/// # Return
/// The `Html` object containing the DOM representation of the experience
/// section
#[function_component(Experience)]
pub fn experience() -> Html {
    let experience_style = css!(
    "
    text-align: center;

    p {
        width: min(80vw, 40rem);
        margin-left: auto;
        margin-right: auto;
    }
    "
    );

    html! {
        <div class={classes!(experience_style, "section", "bg-1", "text-1")}>
            <div class={classes!("divider-top", "bg-2")}/>
            <p class={"text-header"}>{"Where I've Worked"}</p>
            <div class={classes!("divider-bottom", "bg-2")}/>
        </div>
    }
}
