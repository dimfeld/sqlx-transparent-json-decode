CREATE TABLE data (id bigint PRIMARY KEY, data jsonb NOT NULL);

INSERT INTO
  data (id, data)
VALUES
  (1, '{"s": "abc","i": 123}'),
  (2, '{"s": "def","i": 456}');
