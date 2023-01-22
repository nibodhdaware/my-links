use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Link {
    link: String,
    text: String,
    img: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let links = vec![
        Link {
            link: "https://www.youtube.com/@nibodhdaware".to_string(),
            text: "Youtube".to_string(),
            img: "/img/youtube-min.png".to_string(),
        },
        Link {
            link: "https://github.com/nibodhdaware".to_string(),
            text: "GitHub".to_string(),
            img: "/img/github-min.png".to_string(),
        },
        Link {
            link: "https://www.linkedin.com/in/nibodhdaware/".to_string(),
            text: "LinkedIn".to_string(),
            img: "/img/linkedin.png".to_string(),
        },
        Link {
            link: "https://twitter.com/nibodhdaware".to_string(),
            text: "Twitter".to_string(),
            img: "/img/twitter-min.png".to_string(),
        },
        Link {
            link: "https://www.instagram.com/nibodhjdaware".to_string(),
            text: "Instagram".to_string(),
            img: "/img/instagram-min.png".to_string(),
        },
    ];

    let links = links
        .iter()
        .map(|link| {
            html! {
                <ul>
                    <li>
                        <a href={link.link.clone()} target="_blank"><img src={link.img.clone()} alt={link.text.clone()} width="20"/>{&link.text.clone()}</a>
                    </li>
                </ul>
            }
        })
        .collect::<Html>();

    html! {
        <>
            <header>
                <img src="/img/nibodh-min.png" alt="image" width="100"/>
                <h1>{"@nibodhdaware"}</h1>
            </header>
            {links}
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
