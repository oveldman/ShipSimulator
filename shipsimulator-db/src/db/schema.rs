table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        encrypted_password -> Varchar,
        cookie_id -> Varchar,
        login_expired_at -> Timestamptz,
    }
}
