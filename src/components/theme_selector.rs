//! Includes methods and structs for the `ThemeSelector` component to switch
//! between color themes
use stylist::css;
use yew::{
    Callback,
    classes,
    function_component,
    Html,
    html,
    MouseEvent,
    use_context,
};
use crate::{
    AppContext,
    ThemeAction,
};

/// Component which displays and operates the theme selector
#[function_component(ThemeSelector)]
pub fn theme_selector() -> Html {
    let context = use_context::<AppContext>().expect("App context missing!");

    let slide_offset = match context.theme.cur_theme {
        "day" => "100%",
        _ => "0%",
    };

    let theme_swap = |theme: ThemeAction| -> Callback<MouseEvent> {
        Callback::from(move |_| context.theme.dispatch(theme.clone()))
    };

    // This is defined in a css! macro to allow for the animation of the slide
    // bar, so the x offset variable of the selector can be passed into the css
    let slide_style = css!(
        "
        width: min(12vw, 6rem);
        height: min(4.8vw, 2.4rem);
        border-radius: min(1.2vw, 0.7rem);
        background-color: var(--backdrop);
        position: relative;
        animation: fade-in 1s;

        .switch-block {
            position: relative;
            z-index: 1;
            height: 100%;
            width: 50%;
            display: flex;
            justify-content: center;
            align-items: center;
            cursor: pointer;
            color: var(--bg);
        }

        .night-block {
            float: left;
        }

        .day-block {
            float: right;
        }

        .switch-selector {
            position: absolute;
            background-color: var(--switch);
            height: 100%;
            width: 50%;
            border-radius: min(1vw, 0.5rem);
            left: 0;
            transform: translateX(${slide_offset});
            transition: all 0.3s ease-out;
        }

        @media (max-width: 100rem) {
            margin-left: auto;
            margin-right: auto;
        }
        ",
        slide_offset = slide_offset,
    );

    html! {
        <div class={slide_style}>
            <div 
                onclick={theme_swap.clone()(ThemeAction::Night)} 
                class={classes!(
                    "bi",
                    "bi-moon-fill",
                    "switch-block",
                    "night-block",
                    "text-norm"
                )}
            />
            <div
                onclick={theme_swap.clone()(ThemeAction::Day)}
                class={classes!(
                    "bi",
                    "bi-sun-fill",
                    "switch-block",
                    "day-block",
                    "text-norm"
                )}
            />
            <div class={"switch-selector"}/>
        </div>
    }
}
