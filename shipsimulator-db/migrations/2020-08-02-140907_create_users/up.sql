-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL,
    encrypted_password VARCHAR(200) NOT NULL,
    cookie_id VARCHAR(50) NOT NULL,
    login_expired_at timestamptz NOT NULL
)