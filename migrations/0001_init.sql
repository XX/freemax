CREATE TYPE participant_kind AS ENUM ('person', 'organization');
CREATE TYPE member_role AS ENUM ('owner', 'admin', 'member');
CREATE TYPE membership_status AS ENUM ('pending', 'active', 'rejected', 'left', 'banned');
CREATE TYPE listing_kind AS ENUM ('need', 'offer');
CREATE TYPE listing_status AS ENUM ('active', 'reserved', 'closed', 'archived');
CREATE TYPE response_status AS ENUM ('proposed', 'accepted', 'declined', 'completed', 'cancelled');

CREATE TABLE participants (
    id          UUID PRIMARY KEY,
    kind        participant_kind NOT NULL,
    name        TEXT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE circles (
    id          UUID PRIMARY KEY,
    name        TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    owner_id    UUID NOT NULL REFERENCES participants(id),
    rules       JSONB NOT NULL DEFAULT '{}',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE memberships (
    circle_id      UUID NOT NULL REFERENCES circles(id) ON DELETE CASCADE,
    participant_id UUID NOT NULL REFERENCES participants(id) ON DELETE CASCADE,
    role           member_role NOT NULL DEFAULT 'member',
    status         membership_status NOT NULL DEFAULT 'pending',
    joined_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (circle_id, participant_id)
);

CREATE TABLE listings (
    id          UUID PRIMARY KEY,
    kind        listing_kind NOT NULL,
    author_id   UUID NOT NULL REFERENCES participants(id),
    title       TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    category    TEXT,
    status      listing_status NOT NULL DEFAULT 'active',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at  TIMESTAMPTZ
);

CREATE TABLE listing_circles (
    listing_id UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
    circle_id  UUID NOT NULL REFERENCES circles(id) ON DELETE CASCADE,
    PRIMARY KEY (listing_id, circle_id)
);

CREATE TABLE responses (
    listing_id   UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
    responder_id UUID NOT NULL REFERENCES participants(id),
    message      TEXT NOT NULL DEFAULT '',
    status       response_status NOT NULL DEFAULT 'proposed',
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (listing_id, responder_id)
);

CREATE INDEX listing_circles_circle_idx ON listing_circles (circle_id);
CREATE INDEX listings_status_kind_idx ON listings (status, kind);
