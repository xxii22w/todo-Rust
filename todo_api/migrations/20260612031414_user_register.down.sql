-- 回滚时必须反向操作：先删外键，再删字段，最后删表
ALTER TABLE todos DROP FOREIGN KEY fk_todos_user_id;
ALTER TABLE todos DROP COLUMN user_id;

DROP TABLE users;
