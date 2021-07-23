use diesel::table;

table! {
    project (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
    }
}
