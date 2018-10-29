-- Your SQL goes here

CREATE TABLE message_types (
  type_name TEXT PRIMARY KEY
);

INSERT INTO message_types VALUES ('text');

CREATE TABLE content (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  content BYTEA NOT NULL
);

CREATE TABLE messages (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  sender UUID REFERENCES users(id),
  -- We can't make a foreign key on an array, so for now we have to stick with server-side check
  recipients UUID[] NOT NULL,
  message TEXT,
  contents_type TEXT NOT NULL REFERENCES message_types,
  -- The same thing as with recipients field
  content UUID[],
  date_sent DATE DEFAULT now(),
  deleted BOOL DEFAULT false
)