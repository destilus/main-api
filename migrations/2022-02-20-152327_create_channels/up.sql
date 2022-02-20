CREATE TABLE channels 
(
  id UUID NOT NULL DEFAULT uuid_generate_v4(), 
  owner_id UUID NOT NULL references users(id),
  title VARCHAR(64) NOT NULL,
  summary VARCHAR(512) NULL,
  frequency VARCHAR(32) NOT NULL,
  exclusivity VARCHAR(32) NOT NULL,
  subscription_price NUMERIC(24, 18) NULL, -- ETH has 18 decimals
  price_currency VARCHAR(8) NULL, -- THIS MAY CHANGE TO ACCEPT MORE THAN ONE CURRENCY
  created_at timestamp NOT NULL DEFAULT NOW(),
  updated_at timestamp NOT NULL DEFAULT NOW(),
  CONSTRAINT channels_pkey PRIMARY KEY ( id ),
  CONSTRAINT channel_frequency_check CHECK (frequency IN (
    'ONCE', 'HOUR', 'DAY', 'WEEK', 'MONTHL', 'QUARTER', 'SEMESTER', 'YEAR' 
  )),
  CONSTRAINT channel_exclusivity_check CHECK (exclusivity IN (
    'FREE', 'SUBSCRIPTION', 'MIXED'
  )),
  CONSTRAINT chn_currency_check CHECK (price_currency IN (
    'BRL', 'USD', 'USDT', 'USDC', 'BTC', 'ETH' 
  ))
);

CREATE TRIGGER channels_timestamps
BEFORE UPDATE ON channels
FOR EACH ROW
EXECUTE PROCEDURE set_updated_at();