//! Contains the `About` component with the photo and description
use stylist::css;
use yew::{
    classes,
    function_component,
    Html,
    html,
};

/// Component which displays the about information
///
/// # Return
/// The `Html` object containing the DOM representation of the about section
#[function_component(About)]
pub fn about() -> Html {
    let about_style = css!(
        "
        text-align: center;

        img {
            width: min(40vw, 20rem);
            height: min(40vw, 20rem);

            border: min(1vw, 0.5rem) solid var(--bg-2);
            border-radius: 50%;

            object-fit: cover;
        }

        #description {
            width: min(80vw, 40rem);
            margin-left: auto;
            margin-right: auto;
        }
        "
    );

    html! {
        <div class={classes!(about_style, "section", "bg-1", "text-1")}>
            <img src="./assets/profile.jpg" alt="Me on a VLA antenna!"/>
            <p class={"text-header"}>{"👋 Hi, I'm Nathan!"}</p>

            <div id={"description"}>
                <p class={"text-norm"}>
                    {concat!(
                        "That's me on top of an antenna at the Very Large Array! ",
                        "I'm a system administrator turned software engineer, ",
                        "currently with the National Radio Astronomy Observatory.",
                    ) }
                </p>
            </div>
            <div class={classes!("divider-bottom", "bg-2")}/>
        </div>
    }
}
