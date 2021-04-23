table! {
    branches (id) {
        id -> Binary,
        title -> Text,
    }
}

table! {
    build_status (id) {
        id -> Integer,
        title -> Text,
    }
}

table! {
    commits (rev_hash) {
        rev_hash -> Text,
        branch_id -> Nullable<Integer>,
        pull_request_id -> Nullable<Integer>,
    }
}

table! {
    drvs (drv_path) {
        drv_path -> Text,
        attribute -> Text,
        previous_drv -> Nullable<Text>,
        platform_id -> Integer,
        commit_rev_hash -> Text,
        build_status_id -> Integer,
    }
}

table! {
    platforms (id) {
        id -> Integer,
        platform -> Text,
    }
}

table! {
    pull_requests (id) {
        id -> Integer,
    }
}

joinable!(commits -> branches (branch_id));
joinable!(commits -> pull_requests (pull_request_id));
joinable!(drvs -> build_status (build_status_id));
joinable!(drvs -> commits (commit_rev_hash));
joinable!(drvs -> platforms (platform_id));

allow_tables_to_appear_in_same_query!(
    branches,
    build_status,
    commits,
    drvs,
    platforms,
    pull_requests,
);
