-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  password_hash VARCHAR NOT NULL,
  autologin INT DEFAULT NULL,
  administrator BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE planes (
  id SERIAL PRIMARY KEY,
  displayname VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

INSERT INTO planes (displayname, description) VALUES
  ('The Spirit Plane', 'An ever-shifting landscape inhabited by the spirits.'),
  ('The Mortal Plane', 'The primary realm of mortals.');

CREATE TABLE continents (
  id SERIAL PRIMARY KEY,
  plane_id INT NOT NULL REFERENCES planes(id),
  displayname VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

INSERT INTO continents (plane_id, displayname, description) VALUES
  (1, 'The Firmament', 'The Firmament is a place in the spirit realm where only the spirits may go.'),
  (2, 'Rinora', 'Rinora, the primary continent for mortals.');

CREATE TABLE areas (
  id SERIAL PRIMARY KEY,
  continent_id INT NOT NULL REFERENCES continents(id),
  displayname VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

INSERT INTO areas (continent_id, displayname, description) VALUES
  (1, 'The Infinite Mirror', 'Lapping against the shores of The Firmament, the Infinite Mirror stretches beyond space and time.'),
  (1, 'The Celestial Refuge', 'The gathering place of spiritis, a neutral ground for discussion, learning, and thought.'),
  (2, 'The Vast Wilderness', 'You are lost amongst the plains, which stretch beyond your sight.');

CREATE TABLE environments (
  id SERIAL PRIMARY KEY,
  displayname VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

INSERT INTO environments (displayname, description) VALUES
  ('Shoreline', 'On the shore of a body of water.'),
  ('Plains', 'Amid grassy plains.');

CREATE TABLE rooms (
  id SERIAL PRIMARY KEY,
  area_id INT NOT NULL REFERENCES areas(id),
  shortname VARCHAR NOT NULL UNIQUE,
  displayname VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  environment_id VARCHAR NOT NULL
);

(1, "Before the steps leading up to The Celestial Refuge")

CREATE TABLE exits (
  id SERIAL PRIMARY KEY,
  from_room_id INT NOT NULL REFERENCES rooms(id),
  to_room_id INT NOT NULL REFERENCES rooms(id),
  hidden BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE characters (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL REFERENCES users(id),
  shortname VARCHAR NOT NULL UNIQUE,
  description VARCHAR NOT NULL,
  current_room_id INT NOT NULL REFERENCES rooms(id)
);