CREATE TABLE bets 
(
  id UUID NOT NULL DEFAULT uuid_generate_v4(), 
  creator_id UUID NOT NULL references users(id),
  title VARCHAR(64) NOT NULL,
  summary VARCHAR(256) NULL,
  curr_status VARCHAR(32) NOT NULL,
  category VARCHAR(32) NOT NULL,
  bids_limit_date timestamp NOT NULL,
  result_released_at timestamp NULL,
  created_at timestamp NOT NULL DEFAULT NOW(),
  updated_at timestamp NOT NULL DEFAULT NOW(),
  CONSTRAINT bets_pkey PRIMARY KEY ( id ),
  CONSTRAINT bets_status_check CHECK (curr_status IN (
    'DRAFT', 'ONGOING', 'CLOSED'
  )),
  CONSTRAINT bets_category_check CHECK (category IN (
    'PREDICTION', 'STATEMENT', 'DEFIANCE_BET'
  ))
);

CREATE TRIGGER bets_timestamps
BEFORE UPDATE ON bets
FOR EACH ROW
EXECUTE PROCEDURE set_updated_at();