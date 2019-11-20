table! {
    post (id) {
        id -> Bpchar,
        title -> Varchar,
        content -> Text,
        created_timestamp -> Timestamptz,
        updated_timestamp -> Timestamptz,
    }
}
