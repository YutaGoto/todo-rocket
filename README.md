# todo-rocket

## 始め方

dieselのCLIツールをいれる

```sh
cargo install diesel_cli
```

Migrationタスクを実行する

```sh
diesel migration run --database-url ./db/db.sqlite
```

DBをリセットするときは redo する

```sh
diesel migration redo --database-url ./db/db.sqlite
```

## サーバの起動

```sh
cargo run
```

## テスト

```sh
cargo test
```
