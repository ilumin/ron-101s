-- migrate:up

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL UNIQUE,
  hashed_password VARCHAR NOT NULL,
  reset_password_selector VARCHAR,
  reset_password_sent_at TIMESTAMP,
  reset_password_validator_hash VARCHAR,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO users(email, hashed_password) VALUES('test1@test1.com', 'aasdsaddasad');
INSERT INTO users(email, hashed_password) VALUES('test2@test2.com', 'aasdsaddasad');
INSERT INTO users(email, hashed_password) VALUES('test3@test3.com', 'aasdsaddasad');

CREATE TABLE sessions (
  id SERIAL PRIMARY KEY,
  session_verifier VARCHAR NOT NULL,
  user_id INT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  opt_code_encrypted VARCHAR NOT NULL,
  opt_code_attemps INTEGER NOT NULL DEFAULT 0,
  opt_code_confirmed BOOLEAN NOT NULL DEFAULT false,
  opt_code_sent BOOLEAN NOT NULL DEFAULT false
);

-- migrate:down

DROP TABLE users;
DROP TABLE sessions;
