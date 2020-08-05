table! {
    claim_user (id) {
        id -> Int4,
        claim_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    claims (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        encrypted_password -> Varchar,
        cookie_id -> Varchar,
        login_expired_at -> Timestamptz,
    }
}

joinable!(claim_user -> claims (claim_id));
joinable!(claim_user -> users (user_id));

allow_tables_to_appear_in_same_query!(
    claim_user,
    claims,
    users,
);
