CREATE TABLE users 
(
  id UUID NOT NULL DEFAULT uuid_generate_v4(), 
  email VARCHAR NOT NULL,
  pwd VARCHAR NOT NULL,
  verified BOOLEAN NOT NULL DEFAULT false,
  created_at timestamp NOT NULL DEFAULT NOW(),
  updated_at timestamp NOT NULL DEFAULT NOW(),
  CONSTRAINT users_pkey PRIMARY KEY ( id ),
  CONSTRAINT users_email_uk UNIQUE ( email )
);

CREATE TRIGGER users_timestamps
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE set_updated_at();