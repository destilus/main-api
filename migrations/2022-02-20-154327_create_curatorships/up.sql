CREATE TABLE curatorships 
(
  id UUID NOT NULL DEFAULT uuid_generate_v4(), 
  curator_id UUID NOT NULL references users(id),
  defied_curatorship_id UUID NULL references curatorships(id),
  channel_id UUID NULL references channels(id),
  curator_bet_id UUID NULL references bets(id),
  title VARCHAR(64) NOT NULL,
  subtitle VARCHAR(128) NOT NULL,
  hero_image_url VARCHAR(128) NOT NULL,
  body VARCHAR(128) NOT NULL,
  curr_status VARCHAR(32) NOT NULL,
  category VARCHAR(32) NOT NULL,
  exclusivity VARCHAR(32) NOT NULL,
  priority_order VARCHAR(16) NOT NULL,
  price_currency VARCHAR(128) NULL,
  single_price NUMERIC(24, 18) NULL, -- ETH has 18 decimals
  verified BOOLEAN NOT NULL DEFAULT false,
  previews_count SMALLINT NOT NULL DEFAULT 0,
  published_at timestamp NULL,
  created_at timestamp NOT NULL DEFAULT NOW(),
  updated_at timestamp NOT NULL DEFAULT NOW(),
  CONSTRAINT curatorships_pkey PRIMARY KEY ( id ),
  CONSTRAINT crt_status_check CHECK (curr_status IN (
    'PUBLISHED', 'DRAFT', 'UNPUBLISHED'
  )),
  CONSTRAINT crt_category_check CHECK (category IN (
    'NFTS','CRYPTO', 'INVESTMENT', 'PRODUCTS', 'NEWS', 'MOVIES', 'BOOKS', 'VIDEOS'
  )),
  CONSTRAINT crt_exclusivity_check CHECK (exclusivity IN (
    'FREE', 'SUBSCRIPTION', 'SUBSCRIPTION_OR_SINGLE'
  )),
  CONSTRAINT crt_priority_order_check CHECK (priority_order IN (
    'ASC', 'DESC' 
  )),
  CONSTRAINT crt_currency_check CHECK (price_currency IN (
    'BRL', 'USD', 'USDT', 'USDC', 'BTC', 'ETH' 
  ))
);



-- CREATE TABLE curatorships 
-- (
--   id UUID NOT NULL DEFAULT uuid_generate_v4(), 
--   curator_id UUID NOT NULL references users(id),
--   defied_curatorship_id UUID NULL references curatorships(id),
--   channel_id UUID NULL references channels(id),
--   curator_bet_id UUID NULL references bets(id),
  -- title VARCHAR(64) NOT NULL,
  -- subtitle VARCHAR(128) NOT NULL,
  -- hero_image_url VARCHAR(128) NOT NULL,
  -- body VARCHAR(128) NOT NULL,
  -- curr_status VARCHAR(32) NOT NULL,
  -- category VARCHAR(32) NOT NULL,
  -- frequency VARCHAR(32) NOT NULL,
  -- exclusivity VARCHAR(32) NOT NULL,
  -- single_price NUMERIC(24, 18) NULL, -- ETH has 18 decimals
  -- price_currency VARCHAR(128) NOT NULL,
  -- previews_count SMALLINT NOT NULL DEFAULT 0,
  -- priority_order VARCHAR(16) NOT NULL,
  -- verified BOOLEAN NOT NULL DEFAULT false,
  -- published_at timestamp NULL,
  -- created_at timestamp NOT NULL DEFAULT NOW(),
  -- updated_at timestamp NOT NULL DEFAULT NOW(),
  -- CONSTRAINT curatorships_pkey PRIMARY KEY ( id ),
  -- CONSTRAINT crt_status_check CHECK (curr_status IN (
  --   'PUBLISHED', 'DRAFT', 'UNPUBLISHED'
  -- )),
  -- CONSTRAINT crt_category_check CHECK (category IN (
  --   'NFTS','CRYPTO', 'INVESTMENT', 'PRODUCTS', 'NEWS', 'MOVIES', 'BOOKS', 'VIDEOS'
  -- )),
  -- CONSTRAINT crt_exclusivity_check CHECK (exclusivity IN (
  --   'FREE', 'SUBSCRIPTION', 'SUBSCRIPTION_OR_SINGLE'
  -- )),
  -- CONSTRAINT crt_priority_order_check CHECK (priority_order IN (
  --   'ASC', 'DESC' 
  -- )),
  -- CONSTRAINT crt_currency_check CHECK (price_currency IN (
  --   'BRL', 'USD', 'USDT', 'USDC', 'BTC', 'ETH' 
  -- ))
-- );


CREATE TRIGGER curatorships_timestamps
BEFORE UPDATE ON curatorships
FOR EACH ROW
EXECUTE PROCEDURE set_updated_at();