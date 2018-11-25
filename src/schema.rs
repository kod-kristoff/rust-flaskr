table! {
    posts (id) {
        id -> Integer,
        author_id -> Integer,
        created -> Timestamp,
        title -> Text,
        body -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

joinable!(posts -> users (author_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
