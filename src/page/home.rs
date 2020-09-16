use seed::{prelude::*, *};
use crate::Urls;

pub fn view<Ms>(base_url: &Url) -> Node<Ms> {
    section![C!["hero", "is-medium", "ml-6"],
        div![C!["hero-body"],
            h1![C!["title", "is-size-1"],
                "Time Tracker",
            ],
            a![attrs!{At::Href => "https://seed-rs.org/"},
                h2![C!["subtitle", "is-size-3"],
                    "seed-rs.org"
                ]
            ],
            a![C!["button", "is-primary", "mt-5", "is-size-5"], attrs!{At::Href => Urls::new(base_url).time_tracker()},
                strong!["Go to Time Tracker"],
            ],
        ]
    ]
}
