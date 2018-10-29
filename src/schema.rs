table! {
    content (id) {
        id -> Uuid,
        content -> Bytea,
    }
}

table! {
    messages (id) {
        id -> Uuid,
        sender -> Nullable<Uuid>,
        recipients -> Array<Uuid>,
        message -> Nullable<Text>,
        contents_type -> Text,
        content -> Nullable<Array<Uuid>>,
        date_sent -> Nullable<Date>,
        deleted -> Nullable<Bool>,
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
    content,
    messages,
    message_types,
    users,
);
