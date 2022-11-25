-- Your SQL goes here

CREATE TABLE media (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    artist TEXT,
    album TEXT,
    location TEXT NOT NULL UNIQUE
);

