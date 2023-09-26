-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  password_hash VARCHAR NOT NULL,
  autologin INT DEFAULT NULL,
  administrator BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE planes (
  id SERIAL PRIMARY KEY,
  shortname VARCHAR NOT NULL,
  displayname VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

CREATE TABLE continents (
  id SERIAL PRIMARY KEY,
  plane_id INT NOT NULL REFERENCES planes(id),
  shortname VARCHAR NOT NULL,
  displayname VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

CREATE TABLE areas (
  id SERIAL PRIMARY KEY,
  continent_id INT NOT NULL REFERENCES continents(id),
  shortname VARCHAR NOT NULL,
  displayname VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

CREATE TABLE environments (
  id SERIAL PRIMARY KEY,
  shortname VARCHAR NOT NULL,
  displayname VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

CREATE TABLE rooms (
  id SERIAL PRIMARY KEY,
  area_id INT NOT NULL REFERENCES areas(id),
  shortname VARCHAR NOT NULL,
  displayname VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  environment_id VARCHAR NOT NULL
);

CREATE TABLE exits (
  id SERIAL PRIMARY KEY,
  from_room_id INT NOT NULL REFERENCES rooms(id),
  to_room_id INT NOT NULL REFERENCES rooms(id),
  hidden BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE characters (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL REFERENCES users(id),
  shortname VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  current_room_id INT NOT NULL REFERENCES rooms(id)
);