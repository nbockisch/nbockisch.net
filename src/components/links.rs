//! Includes methods and structs for the `Links` component to display contact
//! links
use yew::{
    function_component,
    Html,
    html,
    Properties,
};

/// Contains data for a link to be displayed on the page
///
/// # Parameters
/// * `icon_class` - A `String` with the Bootstrap classes for the icon that
/// will represent the link
/// * `mouseover_text` - A `String` with the text that appears when you mouse
/// over the link
/// * `url` - A `String` with the url the link directs to
pub struct Link {
    icon_class: String, // The Bootstrap icon class
    mouseover_text: String,
    url: String,
}

impl PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        self.icon_class.eq(&other.icon_class)
        && self.mouseover_text.eq(&other.mouseover_text)
        && self.url.eq(&other.url)
    }
}

impl Link {
    /// Creates a new `Link` from given parameters
    ///
    /// # Arguments
    /// * `icon_class` - A `ToString` implementing type with the Bootstrap
    /// classes for the icon representing the link
    /// * `mouseover_text` - A `ToString` implementing type with the text that
    /// appears when you mouse over the link
    /// * `url` - A `ToString` implementing type with the url the link directs
    /// to
    ///
    /// # Return
    /// A `Link` constructed from the given arguments
    pub fn new<S: ToString>(icon_class: &S, mouseover_text: &S, url: &S) 
        -> Self {

        let icon_class = icon_class.to_string().clone();
        let mouseover_text = mouseover_text.to_string().clone();
        let url = url.to_string().clone();

        Self {icon_class, mouseover_text, url}
    }

    /// Outputs the link HTML to be displayed
    ///
    /// # Return
    /// An `Html` object with the HTML link
    pub fn get_html(&self) -> Html {
        html! {
            <a
                class={&self.icon_class}
                target="_blank"
                title={self.mouseover_text.clone()}
                href={self.url.clone()}
            />
        }
    }
}

/// Contains the properties for the `Links` component
///
/// # Parameters
/// * `links` - A `Vec<Link>` containing the `Link` objects to display in the
/// component
#[derive(Properties, PartialEq)]
pub struct LinkProps {
    #[prop_or_default]
    pub links: Vec<Link>,
}

/// Component which displays links from `Link` objects
///
/// # Arguments
/// * `props` - A `&LinkProps` object with the `Link` objects to display
///
/// # Return
/// The `Html` object containing the DOM representation of the links
#[function_component(Links)]
pub fn links(link_props: &LinkProps) -> Html {
    html! {
        <div class={"links"}>
            <ul>
                {
                    link_props.links
                        .iter()
                        .map(
                            |link| html!{
                                <li class={"text-subheader"}>
                                    {link.get_html()}
                                </li>
                            }
                        )
                        .collect::<Html>()
                }
            </ul>
        </div>
    }
}
