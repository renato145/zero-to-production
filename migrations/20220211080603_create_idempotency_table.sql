CREATE TYPE header_pair AS (
    name TEXT,
    value BYTEA
);

CREATE TYPE http_response AS (
    status_code SMALLINT,
    headers header_pair[],
    body BYTEA
);

CREATE TABLE idempotency (
   user_id uuid NOT NULL REFERENCES users(user_id),
   idempotency_key TEXT NOT NULL,
   http_response http_response NOT NULL,
   PRIMARY KEY(user_id, idempotency_key)
);
