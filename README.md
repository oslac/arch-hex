# README

Example of ports and adapters / hexagon / onion architecture in Rust.

- `cargo run --bin cli` to build & run the CLI application; read-only.
- `cargo run --bin http` to build & run the http API; read-write.

0. `cargo run --bin http`
1. `curl -d '{"id":"2", "symbol":"FB"}' -H "Content-Type: application/json" -X POST http://localhost:8080/stonk/`
2. `curl localhost:8080/stonk/`
3. `curl localhost:8080/stonk/2`
4. `curl -X DELETE localhost:8080/stonk/2`

```js
src
├── bin
│   ├── cli
│   └── http
│
├── domain
│   ├── create_stock
│   ├── delete_stock
│   ├── get_stock
│   └── model
│
├── ports
│   ├── create_stock
│   ├── delete_stock
│   └── get_stock
│
└── services
    └── db
        ├── imm_db
        └── lite_db
```
