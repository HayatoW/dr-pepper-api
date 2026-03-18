# Dr Pepper Backend

🧑‍⚕️🌶️🥤

## 技術

- Rust 言語
- [axum](https://docs.rs/axum/latest/axum/) Web App
- [async-graphql](https://docs.rs/async-graphql/latest/async_graphql/) GraphQL Server

## ソフトウェアアーキテクチャ

- クリーンアーキテクチャ
- Event Sourcing
- CQRS (Command Query Responsibility Segregation)

## インフラストラクチャ

- Cloudflare Workers (Tokio 関係で難しいかも)
- Supabase
  - Auth
  - Database

## 開発

```sh
# 依存ライブラリの追加
cargo add <crate_name> -package <package_name>

# API の実行
cargo run -package api
```
