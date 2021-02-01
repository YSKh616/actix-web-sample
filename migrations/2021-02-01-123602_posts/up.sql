CREATE TABLE posts
(
    id SERIAL PRIMARY KEY,
    user_id Integer NOT NULL,
    title VARCHAR(64) NOT NULL,
    body TEXT,
    created_at Datetime,
    updated_at Datetime
);
