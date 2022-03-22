CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  curator_id UUID NULL references users(id),
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
);