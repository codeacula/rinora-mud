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
  shortname VARCHAR NOT NULL UNIQUE,
  displayname VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

INSERT INTO planes (shortname, displayname, description) VALUES
  ('plane_divine', 'The Divine Plane', 'The plane of the Gods, where only the immortal may tread.'),
  ('plane_spirit', 'The Spirit Plane', 'An ever-shifting landscape inhabited by the spirits.'),
  ('plane_mortal', 'The Mortal Plane', 'The primary realm of mortals.');

CREATE TABLE continents (
  id SERIAL PRIMARY KEY,
  plane_id INT NOT NULL REFERENCES planes(id),
  shortname VARCHAR NOT NULL UNIQUE,
  displayname VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

INSERT INTO continents (plane_id, shortname, displayname, description) VALUES
  (1, 'continent_saleva', 'Saleva, Realm of the Divine', 'Nothing is known of Saleva other than it''s where mortals believe the Divine live.'),
  (2, 'continent_firmament', 'The Firmament', 'The Firmament is a place in the spirit realm where only the spirits may go.'),
  (3, 'continent_rinora', 'Rinora', 'Rinora, the primary continent for mortals.');

CREATE TABLE areas (
  id SERIAL PRIMARY KEY,
  continent_id INT NOT NULL REFERENCES continents(id),
  shortname VARCHAR NOT NULL UNIQUE,
  displayname VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

INSERT INTO areas (continent_id, shortname, displayname, description) VALUES
  (1, 'area_saleva', 'Saleva, Realm of the Divine', 'Nothing is known of Saleva other than it''s where mortals believe the Divine live.'),
  (2, 'area_infinite_mirror', 'The Infinite Mirror', 'Lapping against the shores of The Firmament, the Infinite Mirror stretches beyond space and time.'),
  (2, 'area_celestial_refuge', 'The Celestial Refuge', 'The gathering place of spiritis, a neutral ground for discussion, learning, and thought.'),
  (3, 'area_vast_wilderness', 'The Vast Wilderness', 'You are lost amongst the plains, which stretch beyond your sight.');

CREATE TABLE environments (
  id SERIAL PRIMARY KEY,
  shortname VARCHAR NOT NULL UNIQUE,
  displayname VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

INSERT INTO environments (shortname, displayname, description) VALUES
  ('shoreline', 'Shoreline', 'On the shore of a body of water.'),
  ('plains', 'Plains', 'Amid grassy plains.'),

CREATE TABLE rooms (
  id SERIAL PRIMARY KEY,
  area_id INT NOT NULL REFERENCES areas(id),
  shortname VARCHAR NOT NULL UNIQUE,
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
  shortname VARCHAR NOT NULL UNIQUE,
  description VARCHAR NOT NULL,
  current_room_id INT NOT NULL REFERENCES rooms(id)
);