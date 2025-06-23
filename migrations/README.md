### cara delete all table;

sql
```
DROP SCHEMA public CASCADE;
CREATE SCHEMA public;
GRANT ALL ON SCHEMA public TO imam;
GRANT ALL ON SCHEMA public TO public;
```