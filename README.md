# waifus - small, isolated Scuro Guardiano's api apps

## Work in progress

## Rules for waifus
- Waifus can be written in any language although low memory footprint languages are preffered, for example **Rust** or **Go**.
- On start waifu should send `POST` request to `http://{orchestrator_host}/__internal/hello` with this payload:
  ```json
    {
        "name": "waifu name, with docker should be hostname",
        "host": "waifu host name or url, with docker should be hostname",
        // additional binds are optional
        // orchestrator may not respect them if they break orchestator rules
        // for example if it's invalid bind, it's already taken or it tries to bind
        // to main domain.
        "additionalBinds": {
            "https://link.waifu": "/link"
        }
    }
  ```
  Orchestrator will expose waifu using reverse proxy on `{main_domain}/{waifu_name}`.
- If waifu wants a database, it needs to send `POST` request to `http://{orchestrator_host}/__internal/database` with this payload:
  ```json
    {
        "name": "waifu name",
        "secret": "waifu secret, each waifu should have their own secret key, so orchestrator can trust them"
    }
  ```
  If database doesn't exists, orchestrator will create it and save hash of waifu secret. If it does exists, orchestrator will check validity or waifu secret key and if it's correct it will return response. In both cases the response is the same:
  ```json
    {
        "database": "database_name",
        "user": "database_user",
        "password": "database_password",
        "host": "database_host",
        "port": "database_port"
    }
  ```
  Waifu should cache those values. If secret key is invalid `403 - Forbidden` will be returned and the incident will be reported ^^
- Waifu should expose endpoint `/__internal/health` returning HTTP status `204 - No Content`, so orchestrator and other waifus can check waifu's health status.
- Every path fragment starting from `__internal` won't be exposed by reverse proxy, so for communication inside waifus network prefix endpoints with `__internal` path fragment - for example `/__internal/health`.