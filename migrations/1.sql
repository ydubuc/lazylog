CREATE TABLE users (
    id VARCHAR(255) PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    username_key VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL,
    email_key VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    updated_at BIGINT NOT NULL,
    created_at BIGINT NOT NULL
);