use std::rc::Rc;

use stylist::css;
use yew::{
    ContextProvider,
    classes,
    function_component,
    Html,
    html,
    Reducible,
    UseReducerHandle,
    use_effect_with_deps,
    use_reducer,
};

use gloo_storage::{
    errors::StorageError,
    LocalStorage,
    Storage,
};

mod components;
use components::description;

/// An enum with the themes available for the webpage
#[derive(Clone)]
enum ThemeAction {
    Night,
    Day,
}

/// A struct holding the currently selected theme
///
/// # Params
/// * `cur_theme` - A `&str` with the name of the current theme
#[derive(Debug, PartialEq)]
struct ThemeState {
    cur_theme: &'static str,
}

impl Default for ThemeState {
    /// Retrieves the default theme for the webpage, first the theme saved in
    /// local storage if it exists or the `Night` theme if not
    ///
    /// # Return
    /// A `ThemeState` struct with the default theme
    fn default() -> Self {
        let saved_theme: Result<String, StorageError>
            = LocalStorage::get("theme");

        match &saved_theme {
            Ok(t) => match t.as_str() {
                "day" => Self { cur_theme: "day" },
                _ => Self { cur_theme: "night" },
            },
            _ => Self { cur_theme: "night" },
        }
    }
}

impl Reducible for ThemeState {
    type Action = ThemeAction;

    /// Serves to reduce a `ThemeAction` to a `ThemeState` holding the theme it
    /// represents
    ///
    /// # Arguments
    /// * `action` - A `ThemeAction` with the action to reduce
    ///
    /// # Return
    /// An `Rc<ThemeState>` with `cur_theme` set to the value indicated by the
    /// `action` argument
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        Rc::from(Self {
            cur_theme: match action {
                ThemeAction::Day => "day",
                ThemeAction::Night => "night",
            }
        })
    }
}

/// A struct holding the context, or metadata for the App
#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    theme: UseReducerHandle<ThemeState>,
}

#[function_component(App)]
fn app() -> Html {
    let theme = use_reducer(ThemeState::default);
    // Save the theme on refresh
    use_effect_with_deps(move | theme: &UseReducerHandle<ThemeState> | {
        if let Err(_) = LocalStorage::set("theme", &theme.cur_theme) {
            eprintln!("Error accessing local storage!");
        }
        || ()
    }, theme.clone());

    fn get_theme_class(theme: UseReducerHandle<ThemeState>) -> &'static str {
        match theme.cur_theme {
            "night" => "night",
            _ => "day"
        }
    }

    let base_style = css!(
        "
        width: 100%;
        height: 100%;
        "
    );

    html! {
        <ContextProvider<AppContext> context={AppContext {
            theme: theme.clone(),
        }}>
            <div class={classes!(base_style, get_theme_class(theme))}>
                <description::Description />
            </div>
        </ContextProvider<AppContext>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
