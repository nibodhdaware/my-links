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
            link: "https://www.youtube.com/channel/UCWfwqQyLv4PGJxP6ONWPVwA".to_string(),
            text: "Youtube".to_string(),
            img: "/img/youtube.png".to_string(),
        },
        Link {
            link: "https://github.com/nibodhdaware".to_string(),
            text: "GitHub".to_string(),
            img: "/img/github.png".to_string(),
        },
        Link {
            link: "https://twitter.com/nibodhdaware".to_string(),
            text: "Twitter".to_string(),
            img: "/img/twitter.png".to_string(),
        },
        Link {
            link: "https://www.instagram.com/torpidurite_._".to_string(),
            text: "Instagram".to_string(),
            img: "/img/instagram.png".to_string(),
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
        <img src="img/nibodh.png" alt="Nibodh" />
            <h1><a href="https://nibodhdaware.me" target="_blank">{"@nibodhdaware"}</a></h1>
            </header>
            {links}
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
