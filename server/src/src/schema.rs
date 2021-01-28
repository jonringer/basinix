table! {
    project (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
    }
}

table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    project,
    projects,
);
