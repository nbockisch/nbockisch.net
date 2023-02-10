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
        margin-left: 8%;
        width: 60rem;
        padding: 2rem;

        img {
            width: 15rem;
            max-height: 15rem;

            border: 0.1rem solid var(--text);
            border-radius: 50%;

            object-fit: cover;
            aspect-ratio: 1/1;
        }

        #header {
            display: flex;
            flex-flow: row nowrap;
            align-items: center;
            gap: 5rem;
            margin-bottom: 5rem;
            border-bottom: 1px solid #fff;
        }

        @media (max-width: 80rem) {
            margin-left: auto;
            margin-right: auto;
            width: 90%;
        }
        "
    );

    html! {
        <div class={classes!(about_style, "section", "text")}>

            <div id={"header"}>
                <img src="./assets/profile.jpg" alt="Me on a VLA antenna!"/>
                <p class={"text-header"}>{"👋 Hi, I'm Nathan!"}</p>
            </div>
            <p class={"text-norm"}>
                {concat!(
                    "That's me on top of an antenna at the Very Large Array! ",
                    "I'm a system administrator turned software engineer, ",
                    "currently with the National Radio Astronomy Observatory.",
                ) }
            </p>
        </div>
    }
}
