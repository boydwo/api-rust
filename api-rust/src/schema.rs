// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
    }
}
