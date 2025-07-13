CREATE TABLE "saved_responses"
(
    "id"       serial PRIMARY KEY,
    "request"  varchar UNIQUE NOT NULL,
    "response" varchar        NOT NULL
);
