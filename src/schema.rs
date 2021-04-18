#[macro_use]
use rocket_contrib::databases::diesel::*;
table! {
    users (email) {
        email -> Nullable<Text>,
        hash -> Text,
        firstname -> Text,
        lastname -> Text,
        groups -> Text,
        pfp_link -> Text,
    }
}
