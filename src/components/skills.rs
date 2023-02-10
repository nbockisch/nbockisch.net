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
            desc: concat!("The main app I work on for my day job is deployed ",
                "with Docker")
                .to_string(),
        },
        Skill {
            logo: "./assets/tech-logos/ansible-logo.png".to_string(),
            name: "Ansible".to_string(),
            desc: concat!("I developed Ansible roles to configure new ",
                "machine and web servers at my work, as well as to deploy my ",
                "own machine configuration.").to_string(),
        },
        Skill {
            logo: "./assets/tech-logos/bootstrap-logo.png".to_string(),
            name: "Bootstrap".to_string(),
            desc: concat!("We use Bootstrap as the CSS framework at my work ",
                "and I developed websites for the NM DOT with Bootstrap.")
                .to_string(),
        },
        Skill {
            logo: "./assets/tech-logos/java-logo.png".to_string(),
            name: "Java".to_string(),
            desc: concat!("Java serves as the backend for the NRAO public ",
                "archive and other observatory applications. I was also ",
                "been a TA for an object oriented class using Java and have ",
                "used it for many projects such as an Android application.")
                .to_string(),
        },
        Skill {
            logo: "./assets/tech-logos/python-logo.png".to_string(),
            name: "Python".to_string(),
            desc: concat!("Python is the backend language for the main ",
                "application I work on at my job, as well as many other ",
                "projects. I have also been a grader for a Python class and ",
                "have used Python for personal projects for over a decade.")
                .to_string(),
        },
        Skill {
            logo: "./assets/tech-logos/c-logo.png".to_string(),
            name: "C".to_string(),
            desc: concat!("C was the main language for the curriculum of my ",
                "B.Sc. in Computer Science, and I have used it for projects ",
                "such as implementing a shell and writing a job scheduler.")
                .to_string(),
        },
        Skill {
            logo: "./assets/tech-logos/cpp-logo.png".to_string(),
            name: "C++".to_string(),
            desc: concat!("I used C++ to implement a C compiler during my ",
                "Computer Science degree coursework, and have used it for ",
                "personal projects.").to_string(),
        },
        Skill {
            logo: "./assets/tech-logos/linux-logo.png".to_string(),
            name: "Linux".to_string(),
            desc: concat!("I have spent 4 years as a system administrator ",
                "working with a variety of Linux distros, such as Red Hat, ",
                "Debian, and Arch. I have been using Linux exclusively on my ",
                "personal machines for over 10 years.").to_string(),
        },
        Skill {
            logo: "./assets/tech-logos/git-logo.png".to_string(),
            name: "Git".to_string(),
            desc: concat!("Git is the primary version control system I have ",
                "used, both at my current job and previously.").to_string(),
        },
        Skill {
            logo: "./assets/tech-logos/postgres-logo.png".to_string(),
            name: "PostgreSQL".to_string(),
            desc: concat!("PostgreSQL is the primary RDBMS used by my team at",
                "my current job.").to_string(),
        },
        Skill {
            logo: "./assets/tech-logos/mysql-logo.png".to_string(),
            name: "MySQL".to_string(),
            desc: concat!("MySQL is the RDBMS used for user account storage ",
                "for authentication with the services my team supports. It ",
                "was also the primary RDBMS I used during coursework for my ",
                "Computer Science degree.").to_string(),
        },
        Skill {
            logo: "./assets/tech-logos/rust-logo.png".to_string(),
            name: "Rust".to_string(),
            desc: concat!("Rust is my most-used language for personal ",
                "projects. I have been learning it for a few years, and even ",
                "wrote this website with it!").to_string(),
        },
        Skill {
            logo: "./assets/tech-logos/wasm-logo.png".to_string(),
            name: "Web Assembly".to_string(),
            desc: concat!("I have recently developed an interest in Web ",
                "Assembly and its implications for web development. I used it ",
                "to write this website with the Yew framework!").to_string(),
        },
        Skill {
            logo: "./assets/tech-logos/kubernetes-logo.png".to_string(),
            name: "Kubernetes".to_string(),
            desc: concat!("I'm currently learning Kubernetes and working on ",
                "converting the Dockerized application I primarily work on at ",
                "my job to being managed using it.").to_string(),
        },
        Skill {
            logo: "./assets/tech-logos/typescript-logo.png".to_string(),
            name: "TypeScript".to_string(),
            desc: concat!("TypeScript is the main frontend language for the ",
                "applications my team support at my job, in conjunction with ",
                "Angular.").to_string(),
        },
    ];

    let skills_style = css!(
    "
        text-align: center;
        p {
            width: min(80vw, 40rem);
            margin-left: auto;
            margin-right: auto;
        }

        #skill-grid {
            width: fit-content;
            padding: min(2vw, 2rem);
            margin: min(5vw, 5rem);
            margin-left: auto;
            margin-right: auto;

            display: grid;
            justify-content: center;
            justify-items: center;

            background-color: #eee;
            border-radius: 5%;

            grid-template-columns: repeat(4, min(7vw, 7rem));
            gap: min(5vw, 5rem);
        }
    "
    );

    html! {
        <div class={classes!(skills_style, "section", "bg-2", "text-2")}>
            <div class={classes!("divider-top", "bg-1")}/>
            <p class={"text-header"}>{"My Skills"}</p>
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
            <div class={classes!("divider-bottom", "bg-1")}/>
        </div>
    }
}
