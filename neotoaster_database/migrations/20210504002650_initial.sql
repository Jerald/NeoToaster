-- Add migration script here
CREATE TABLE IF NOT EXISTS Users
(
    id INTEGER PRIMARY KEY NOT NULL,

);

CREATE TABLE IF NOT EXISTS ToastWars_Users
(
    user INTEGER PRIMARY KEY NOT NULL FOREIGN KEY REFERENCES Users(id),
    team INTEGER NOT NULL DEFAULT 0,

);