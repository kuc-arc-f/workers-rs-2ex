# rs-react6_d1

 Version: 0.9.1

 date    : 2025/06/25 

 update :

***

d1 database + Rust + Cloudflare Workers +  React , example

***
* start
```
npm run dev
```

* react-build (other window)
```
npm run build
```
***
### Setup: 
* wrangler.toml
* database_name ,  database_id SET

```
name = "workers-rs-6"
main = "build/worker/shim.mjs"
compatibility_date = "2023-03-22"

assets = { directory = "./public/" }

[build]
command = "cargo install -q worker-build && worker-build --release"

[[d1_databases]]
binding = "DB"
database_name = ""
database_id = ""

```

***

* table: ./schema.sql
```
npx wrangler d1 execute dbname --local --file=./schema.sql
```
***
### Blog

***


