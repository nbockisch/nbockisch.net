//! Includes methods and structs for the description component to display the
//! main web page content
use yew::{classes, function_component, Html, html,};
use super::{
    links::{Link, Links,},
    theme_selector::ThemeSelector,
};

/// Components which displays the main content for the page, including the
/// description text, image, links, and theme selector
///
/// # Return
/// The `Html` object containing the DOM representation of the main web page
/// content
#[function_component(Description)]
pub fn description() -> Html {
    let links = vec![
            Link::new(
                &"bi bi-github",
                &"GitHub",
                &"https://github.com/nbockisch"
            ),
            Link::new(
                &"bi bi-linkedin",
                &"LinkedIn",
                &"https://www.linkedin.com/in/nbockisch"
            ),
            Link::new(
                &"bi bi-envelope-fill",
                &"EMail",
                &"mailto:nbockisch@protonmail.com"
            ),
            Link::new(
                &"bi bi-file-earmark-richtext-fill",
                &"Résumé",
                &"https://github.com/nbockisch/Resume/blob/master/resume.pdf"
            ),
        ];

    html! {
        <div class={classes!("description", "bg")}>
            <ThemeSelector />
            <br/>
            <img src="./assets/profile.jpg" alt="Me on a VLA antenna!"/>
                <div class={classes!("description-text", "text")}>
                    <p class={"text-header"}>
                        {"Hi, I'm Nathan! 👋"}
                    </p>
                    <div class={"text-norm"}>
                        <br/>
                        <p>
                            {"That's me on top of an antenna at the "}
                            <a
                                href="https://public.nrao.edu/telescopes/VLA/"
                                target="_blank"
                            >
                                {"Very Large Array"}
                            </a>
                            {"! I'm a "}
                            <span class={"text-hl"}>
                                {"software engineer "}
                            </span>
                            {"currently with the "}
                            <a
                                href="https://public.nrao.edu/"
                                target="_blank"
                            >
                                {"National Radio Astronomy Observatory"}
                            </a>
                            {" in the "}
                            <span class={"text-hl"}>
                                {"Science Support and Archive "}
                            </span>
                            {"team."}
                        </p>
                        <br/>
                        <p>
                            {"I do "}
                            <span class={"text-hl"}>{"full-stack "}</span>
                            {"development on "}
                            <a
                                href="https://gitlab.nrao.edu/ssa/workspaces"
                                target="_blank"
                            >
                                {"Workspaces"}
                            </a>
                            {concat!(
                            ", the observatory's data processing workflow ",
                            "manager. I also occasionally do work for other ",
                            "projects such as the "
                            )}
                            <a
                                href="https://data.nrao.edu/portal/"
                                target="_blank"
                            >
                                {"NRAO Public Archive "}
                            </a>
                            {"and the "}
                            <a
                                href="https://public.nrao.edu/vlass/"
                                target="_blank"
                            >
                                {"Very Large Array Sky Survey"}
                            </a>
                            {". Previously, I was a "}
                            <span class={"text-hl"}>
                                {"system administrator "}
                            </span>
                            {"with the observatory, maintaining our "}
                            <span class={"text-hl"}>
                                {"Unix/Linux "}
                            </span>
                            {"infrastructure and "}
                            <span class={"text-hl"}>
                                {"HTCondor/SLURM/Torque "}
                            </span>
                            {"data processing computing cluster."}
                        </p>
                        <br/>
                        <p>
                            {"I'm currently doing "}
                            <span class={"text-hl"}>
                                {"dev-ops "}
                            </span>
                            {concat!(
                                "work for my team, including migrating our ",
                                "authentication system to use the ",
                                "observatory's incoming ",
                            )}
                            <span class={"text-hl"}>
                                {"Red Hat IDM "}
                            </span>
                            {"system using "}
                            <span class={"text-hl"}>
                                {"KeyCloak "}
                            </span>
                            {"and researching using "}
                            <span class={"text-hl"}>
                                {"Kubernetes "}
                            </span>
                            {"to manage our "}
                            <span class={"text-hl"}>
                                {"Docker "}
                            </span>
                            {"deployments. "}
                        </p>
                        <br/>
                        <p>
                            {"Outside of work I enjoy learning more about "}
                            <span class={"text-hl"}>
                                {"*nix "}
                            </span>
                            {concat!(
                                "systems, working on programming projects, ",
                                "and learning new languages. Lately I've ",
                                "been enjoying learning ",
                            )}
                            <span class={"text-hl"}>
                                {"Rust"}
                            </span>
                            {". I even "}
                            <a
                                href="https://github.com/nbockisch/nbockisch.net"
                                target="_blank"
                            >
                                {"built this website in it"}
                            </a>
                            {"!"}
                        </p>
                        <br/>
                        <p>
                            {concat!(
                                "If you'd like to get in touch or learn more, ",
                                "feel free to do so with the links below!",
                            )}
                        </p>
                        <br/>
                    </div>
                    <Links links={links}/>
                </div>
        </div>
    }
}
