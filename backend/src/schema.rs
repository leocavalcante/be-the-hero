table! {
    ngos (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        whats_app -> Text,
        city -> Text,
        state -> Text,
    }
}

table! {
    projects (id) {
        id -> Nullable<Integer>,
        ngo_id -> Integer,
        name -> Text,
        description -> Text,
        donation -> Integer,
    }
}

joinable!(projects -> ngos (ngo_id));

allow_tables_to_appear_in_same_query!(
    ngos,
    projects,
);
