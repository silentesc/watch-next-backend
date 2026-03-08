# Watch Next

## Endpoints

#### Endpoints are based on the [TMDB API](https://developer.themoviedb.org/reference).

| Category | Endpoints |
| --- | --- |
| Discover | <ul><li>[Movie](endpoints/discover/movie.md)</li></ul> |
| Genre | <ul><li>[Movie](endpoints/genre/movie.md)</li></ul> |
| Configuration | <ul><li>[Languages](endpoints/configuration/languages.md)</li></ul> |

## Quick Start

### Database

> **NOTE:** `pgadmin` is completely optional if you want to directly look into the db

```yml
services:
  postgres:
    image: postgres:18
    ports:
      - 5432:5432
    environment:
      POSTGRES_USER: my_postgres_user
      POSTGRES_PASSWORD: strong_password
      POSTGRES_DB: name_of_db
      TZ: Etc/UTC
    volumes:
      - ./data/postgres:/var/lib/postgresql

  pgadmin:
    image: dpage/pgadmin4:latest
    ports:
      - 5050:80
    environment:
      # Required by pgAdmin
      PGADMIN_DEFAULT_EMAIL: admin@admin.com
      PGADMIN_DEFAULT_PASSWORD: adminadmin

      # Don't require the user to login
      PGADMIN_CONFIG_SERVER_MODE: 'False'

      # Don't require a "master" password after logging in
      PGADMIN_CONFIG_MASTER_PASSWORD_REQUIRED: 'False'
```

### Watch Next Backend

#### .env file
```dotenv
LOG_LEVEL="INFO" # TRACE, DEBUG, INFO, WARN, ERROR

DATABASE_URL="postgres://my_postgres_user:strong_password@localhost/name_of_db"

SERVE_ADDR="0.0.0.0:3000"
CORS_ALLOWED_ORIGINS="http://localhost:5173,https://api.watch-next.mydomain.com"

TMDB_API_KEY="abcdefghijklmnopqrstuvwxyz"
```

#### Run app
```bash
cargo run
```
