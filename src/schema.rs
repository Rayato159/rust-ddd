// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 128]
        description -> Varchar,
        #[max_length = 256]
        picture -> Nullable<Varchar>,
        price -> Float8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
