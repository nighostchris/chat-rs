-- user have to be wrapped in quote as it crashed with psql reserved word
CREATE TABLE IF NOT EXISTS "user" (
  -- uuid_generate_v4() is only available when uuid-ossp module is enabled
  -- we will be using built-in function gen_random_uuid() for v4 uuid
  -- gen_random_uuid is only available on or after PSQL v13
  -- ref: https://www.postgresql.org/docs/current/functions-uuid.html
  -- ref: https://stackoverflow.com/questions/72144228/sequelize-migration-throws-error-function-uuid-generate-v4-does-not-exist
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  email VARCHAR(100) UNIQUE NOT NULL,
  password VARCHAR(128) NOT NULL,
  verified BOOLEAN NOT NULL DEFAULT false,
  name VARCHAR(50),
  avatar TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
