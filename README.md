## dieselの素振り

マリリントンアプリを作成するための素振り。  

### 手順

install 

```bash
cargo install diesel_cli
```

create

```bash
cargo new diesel_sample
cd diesel_sample
```

Cargo.toml  
ここを `sqlite` ではなくて `mysql` に変えれば MariaDB も使えるはず（知らんけど）

```toml
[package]
name = "diesel_sample"
version = "0.1.0"
authors = ["techno"]

[dependencies]
diesel = { version = "0.15.0", features = ["sqlite"] }
diesel_codegen = { version = "0.15.0", features = ["sqlite"] }
dotenv = "0.9.0"
```

create db

```
echo DATABASE_URL=./sample.db > .env
diesel setup
```

dieselは.envに書かれたURLを元にデータベースに接続するので、データベースのURL(今回はデータベースファイルのパス)を.envに出力後diesel setupを打ちます。


migration 

```bash
diesel migration generate create_posts
```

これで `up.sql` と `down.sql` が生成される。  
以下を記入する。（作りたいテーブルの内容ね）  

```sql
// up.sql
CREATE TABLE posts (
  id INTEGER NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 0
)
```


```sql
// down.sql
DROP TABLE posts
```


```bash
diesel migration run
```
