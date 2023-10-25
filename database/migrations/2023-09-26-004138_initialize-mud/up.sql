-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  password_hash VARCHAR NOT NULL,
  administrator BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO users (username, password_hash, administrator) VALUES
  ('dummy-account1', 'dummy-password1', FALSE),
  ('dummy-account2', 'dummy-password2', FALSE);

CREATE TABLE planes (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

INSERT INTO planes (name, description) VALUES
  ('The Spirit Plane', 'An ever-shifting landscape inhabited by the spirits.'),
  ('The Mortal Plane', 'The primary realm of mortals.');

CREATE TABLE continents (
  id SERIAL PRIMARY KEY,
  plane_id INT NOT NULL REFERENCES planes(id),
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

INSERT INTO continents (plane_id, name, description) VALUES
  (1, 'The Firmament', 'The Firmament is a place in the spirit realm where only the spirits may go.'),
  (2, 'Rinora', 'Rinora, the primary continent for mortals.');

CREATE TABLE areas (
  id SERIAL PRIMARY KEY,
  continent_id INT NOT NULL REFERENCES continents(id),
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

INSERT INTO areas (continent_id, name, description) VALUES
  (1, 'The Infinite Mirror', 'Lapping against the shores of The Firmament, the Infinite Mirror stretches beyond space and time.'),
  (1, 'The Celestial Refuge', 'The gathering place of spiritis, a neutral ground for discussion, learning, and thought.'),
  (2, 'The Vast Wilderness', 'You are lost amongst the plains, which stretch beyond your sight.');

CREATE TABLE environments (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

INSERT INTO environments (name) VALUES
  ('Shoreline'),
  ('Temple'),
  ('Plains');

CREATE TABLE rooms (
  id SERIAL PRIMARY KEY,
  area_id INT NOT NULL REFERENCES areas(id),
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  environment_id INT NOT NULL REFERENCES environments(id)
);

INSERT INTO rooms (area_id, name, description, environment_id) VALUES
(2, 'Lost amongst the vast wilderness', 'You are lost amongst the vast wilderness. The plains stretch to the horizon in every direction.', 3),
(2, 'Lost amongst the vast wilderness', 'You are lost amongst the vast wilderness. The plains stretch to the horizon in every direction.', 3),
(2, 'Lost amongst the vast wilderness', 'You are lost amongst the vast wilderness. The plains stretch to the horizon in every direction.', 3),
(2, 'Lost amongst the vast wilderness', 'You are lost amongst the vast wilderness. The plains stretch to the horizon in every direction.', 3),
(2, 'Lost amongst the vast wilderness', 'You are lost amongst the vast wilderness. The plains stretch to the horizon in every direction.', 3),
(1, 'Before the steps leading up to The Celestial Refuge', 'You stand before the grand steps that lead up to the great', 1),
(1, 'Western shore of The Infinite Mirror', 'You stand on the western shore of The Infinite Mirror. Go east, maybe.', 1);

CREATE TABLE exits (
  id SERIAL PRIMARY KEY,
  from_room_id INT NOT NULL REFERENCES rooms(id),
  to_room_id INT NOT NULL REFERENCES rooms(id),
  direction VARCHAR NOT NULL,
  hidden BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO exits (from_room_id, to_room_id, direction) VALUES
(1, 2, 'n'),
(1, 3, 'e'),
(1, 4, 's'),
(1, 5, 'w'),
(2, 5, 'sw'),
(2, 1, 's'),
(2, 3, 'se'),
(3, 2, 'nw'),
(3, 1, 'w'),
(3, 4, 'sw'),
(4, 5, 'nw'),
(4, 1, 'n'),
(4, 3, 'ne'),
(5, 2, 'ne'),
(5, 1, 'e'),
(5, 4, 'se');

CREATE TABLE characters (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL REFERENCES users(id),
  name VARCHAR NOT NULL UNIQUE,
  description VARCHAR NOT NULL,
  current_room_id INT NOT NULL REFERENCES rooms(id) DEFAULT 1,
  current_hp INT NOT NULL DEFAULT 0,
  current_mp INT NOT NULL DEFAULT 0
);

CREATE TABLE settings (
  id SERIAL PRIMARY KEY,
  support_email VARCHAR NOT NULL,
  default_room INT NOT NULL REFERENCES rooms(id)
);

INSERT INTO settings (support_email, default_room) VALUES
  ('example@example.com', 1);