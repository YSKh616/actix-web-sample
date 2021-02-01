table! {
    posts (id) {
        id -> Unsigned<Bigint>,
        user_id -> Integer,
        title -> Varchar,
        body -> Nullable<Text>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
