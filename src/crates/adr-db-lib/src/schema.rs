// @generated automatically by Diesel CLI.

diesel::table! {
    task_summaries (id) {
        id -> Integer,
        task_id -> Text,
        status -> Text,
        cost -> Text,
        commit_sha -> Text,
        description -> Text,
        ingested_at -> Timestamp,
        source_plan -> Text,
    }
}
