//! Contains the `Skills` component with the list of technologies I use
use stylist::css;
use yew::{
    classes,
    function_component,
    Html,
    html,
};

/// A `struct` which stores the data needed to display a skill
///
/// # Parameters
/// * `logo` - `String` path to the logo image
/// * `name` - `String` with the name of the technology
/// * `desc` - `String` description displayed on mouse hover or tap
struct Skill {
    logo: String,
    name: String,
    desc: String,
}

impl Skill {
    /// Gets the HTML for displaying the `Skill` struct
    ///
    /// # Return
    /// The `Html` object containing the DOM representation of the `Skill`
    /// struct
    fn get_html(&self) -> Html {
        let skill_style = css!(
        "
        width: min(6vw, 6rem);
        height: min(6vw, 6rem);

        background-color: #ddd;
        border-radius: 20%;
        padding: min(0.5vw, 0.5rem);

        opacity: 50%;

        :hover,:active {
            opacity: 100%;
        }
        "
        );

        html! {
            <img
                class={skill_style}
                src={self.logo.clone()}
                alt={self.name.clone()}
            />
        }
    }
}

/// Component which displays the skills information
///
/// # Return
/// The `Html` object containing the DOM representation of the skills section
#[function_component(Skills)]
pub fn skills() -> Html {
    let skill_boxes = vec![
        Skill {
            logo: "./assets/tech-logos/angular-logo.png".to_string(),
            name: "Angular".to_string(),
            desc: "My day job is full-stack development of Angular apps"
                .to_string(),
        },
        Skill {
            logo: "./assets/tech-logos/docker-logo.png".to_string(),
            name: "Docker".to_string(),
            desc: "The main app I work on for my day job is deployed with Docker"
                .to_string(),
        },
    ];

    let skills_style = css!(
    "
        text-align: center;

        #skill-grid {
            display: grid;
            justify-content: center;

            grid-gap: min(1vw, 1rem);
        }
    "
    );

    html! {
        <div class={classes!(skills_style, "section", "bg-2", "text-2")}>
            <div class={classes!("divider-top", "bg-1")}/>
            <p class={"text-header"}>{"Skills"}</p>
            <p class={"text-norm"}>
                {concat!(
                    "Here are some of the technologies I use. Hover over or ",
                    "tap on them to learn more."
                )}
            </p>
            <div id="skill-grid">
                {
                    skill_boxes
                        .iter()
                        .map(|s| s.get_html())
                        .collect::<Html>()
                }
            </div>
        </div>
    }
}
