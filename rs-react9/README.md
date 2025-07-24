# rs-react9

 Version: 0.9.1

 date    : 2025/07/24

 update :

***

workers-rs +  React , API example

***
### API server

https://github.com/kuc-arc-f/workers21/tree/main/react_example

***
### setup

* wrangler.toml
* API_HOST set

```
name = "workers-rs-9"
main = "build/worker/shim.mjs"
compatibility_date = "2023-03-22"

assets = { directory = "./public/" }

[vars]
API_HOST = "https://localhost"

[build]
command = "cargo install -q worker-build && worker-build --release"

[[d1_databases]]
binding = ""
database_name = ""
database_id = ""
```

***
* start
```
npm run build
npm run dev
```

***
### Blog

***


