CREATE TABLE curatorship_subscribers (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    subscriber_id UUID NOT NULL references users(id),
    curatorship_id UUID NOT NULL references curatorships(id),
    subscription_type VARCHAR(32) NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW(),
    updated_at timestamp NOT NULL DEFAULT NOW(),
    CONSTRAINT subscription_type_check CHECK (subscription_type IN (
        'PAID', 'FREE', 'PAID_ONCE'
    )),
    CONSTRAINT curatorship_subscribers_pkey PRIMARY KEY (id)
);
--- It seems that in order for diesel to recognize a column as having a PRIMARY KEY, the primary key attribute must be the last one specified.
--- Try reverting all migrations before running a new one
CREATE TRIGGER curatorship_subscribers_timestamps
BEFORE UPDATE ON curatorship_subscribers
FOR EACH ROW
EXECUTE PROCEDURE set_updated_at();