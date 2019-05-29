CREATE TABLE widgets (
  id SERIAL PRIMARY KEY,
  category_id SERIAL references categories(id) NOT NULL,
  name VARCHAR NOT NULL,
  component_key VARCHAR NOT NULL
)