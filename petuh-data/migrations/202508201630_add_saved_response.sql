CREATE TABLE "saved_responses" (
                                   "id" SERIAL PRIMARY KEY,
                                   "user_id" integer NOT NULL,
                                   "request" varchar UNIQUE NOT NULL,
                                   "response" varchar NOT NULL
);

ALTER TABLE "saved_responses" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("telegram_id");
