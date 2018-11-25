table! {
    contents (id) {
        id -> Uuid,
        content -> Bytea,
    }
}

table! {
    messages (id) {
        id -> Uuid,
        sender -> Uuid,
        recipients -> Array<Uuid>,
        message -> Nullable<Text>,
        contents_type -> Text,
        contents -> Nullable<Array<Uuid>>,
        date_sent -> Timestamp,
        deleted -> Bool,
    }
}

table! {
    message_types (type_name) {
        type_name -> Text,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        password -> Text,
        salt -> Text,
    }
}

joinable!(messages -> message_types (contents_type));
joinable!(messages -> users (sender));

allow_tables_to_appear_in_same_query!(
    contents,
    messages,
    message_types,
    users,
);
