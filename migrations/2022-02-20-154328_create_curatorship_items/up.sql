CREATE TABLE curatorship_items
(
  id UUID NOT NULL DEFAULT uuid_generate_v4(), 
  curatorship_id UUID NOT NULL references curatorships(id),
  priority_order SMALLINT NOT NULL,
  external_ref VARCHAR(128) NULL,
  title VARCHAR(64) NOT NULL,
  subtitle VARCHAR(128) NULL,
  hero_image_url VARCHAR(128) NULL,
  body TEXT NOT NULL,
  created_at timestamp NOT NULL DEFAULT NOW(),
  updated_at timestamp NOT NULL DEFAULT NOW(),
  CONSTRAINT curatorship_items_pkey PRIMARY KEY ( id )
);

CREATE TRIGGER curatorship_items_timestamps
BEFORE UPDATE ON curatorship_items
FOR EACH ROW
EXECUTE PROCEDURE set_updated_at();