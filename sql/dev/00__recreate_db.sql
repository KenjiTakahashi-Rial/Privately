SELECT
    pg_terminate_backend(pid)
FROM
    pg_stat_activity
WHERE
    username = 'dev_user'
    OR db = 'dev_db';

DROP DATABASE IF EXISTS dev_db;

DROP USER IF EXISTS dev_user;

CREATE USER dev_user PASSWORD 'dev_password';

CREATE DATABASE dev_db OWNER dev_user ENCODING = 'UTF-8';