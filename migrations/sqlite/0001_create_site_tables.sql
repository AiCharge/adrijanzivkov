-- The tables the site uses, as Laravel's migrations created them. `IF NOT EXISTS` lets this run
-- against a database Laravel already set up: existing tables and their data are left alone.

CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    email_verified_at DATETIME,
    password VARCHAR NOT NULL,
    remember_token VARCHAR,
    created_at DATETIME,
    updated_at DATETIME
);
CREATE UNIQUE INDEX IF NOT EXISTS users_email_unique ON users (email);

CREATE TABLE IF NOT EXISTS news (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title VARCHAR NOT NULL,
    short_description TEXT NOT NULL,
    main_image VARCHAR NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME,
    updated_at DATETIME,
    slug VARCHAR NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS news_slug_unique ON news (slug);

CREATE TABLE IF NOT EXISTS donation_needs (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT NOT NULL,
    image VARCHAR NOT NULL,
    needed_amount INTEGER NOT NULL,
    created_at DATETIME,
    updated_at DATETIME,
    sticky_content TEXT,
    archived_at DATETIME
);

CREATE TABLE IF NOT EXISTS story_submissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    phone VARCHAR NOT NULL,
    message TEXT NOT NULL,
    status VARCHAR NOT NULL,
    notes TEXT,
    created_at DATETIME,
    updated_at DATETIME
);
