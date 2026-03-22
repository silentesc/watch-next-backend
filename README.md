# Watch Next

## Endpoints

#### Endpoints are based on the [TMDB API](https://developer.themoviedb.org/reference).

| Category | Endpoints |
| --- | --- |
| Discover | <ul><li>[Movie](docs/endpoints/discover/movie.md)</li></ul> |
| Trending | <ul><li>[Movies](docs/endpoints/trending/movies.md)</li></ul> |
| Search | <ul><li>[Movie](docs/endpoints/search/movie.md)</li><li>[Collection](docs/endpoints/search/collection.md)</li></ul> |
| Genre | <ul><li>[Movie](docs/endpoints/genre/movie.md)</li></ul> |
| Configuration | <ul><li>[Languages](docs/endpoints/configuration/languages.md)</li></ul> |
| Movies | <ul><li>[Details](docs/endpoints/movies/details.md)</li><li>[Release Dates](docs/endpoints/movies/release_dates.md)</li><li>[Credits](docs/endpoints/movies/credits.md)</li><li>[Videos](docs/endpoints/movies/videos.md)</li><li>[Recommendations](docs/endpoints/movies/recommendations.md)</li><li>[Similar](docs/endpoints/movies/similar.md)</li></ul> |
| Collections | <ul><li>[Details](docs/endpoints/collections/details.md)</li></ul> |

## Roadmap

### **v1.0** (current)

- **Planned features**
  - 🔜 Lists (e.g. Plan to Watch, Watching, Completed)
  - 🔜 Custom Tags per movie/show (e.g. must-watch)
  - 🔜 Auto updating data in db
  - 🔜 Caching
- **Planned endpoints**
  - 🔜 Discover
    - ✅ Movie
    - 🔜 TV Shows
  - 🔜 Trending
    - 🔜 All
    - ✅ Movie
    - 🔜 TV Shows
  - 🔜 Search
    - ✅ Collection
    - ✅ Movie
    - 🔜 Multi
    - 🔜 Person
    - 🔜 TV Shows
  - ✅ Genres
    - ✅ Movies
    - 🔜 TV Shows
  - ✅ Languages
  - 🔜 People
    - 🔜 Details
    - 🔜 Combined Credits
  - ✅ Collections
    - ✅ Details
  - ✅ Movies
    - ✅ Details
    - ✅ Release Dates
    - ✅ Credits
    - ✅ Videos
    - ✅ Recommendations
    - ✅ Similar
  - 🔜 TV Shows
    - 🔜 ...
  - 🔜 TV Seasons
    - 🔜 ...
  - 🔜 TV Episodes
    - 🔜 ...

### **v1.1** (next)

- 🔜 Analytics
  - 🔜 Total hours watched
  - 🔜 Watch history
  - 🔜 Favorite genres
  - 🔜 Viewing patterns (e.g. prefer short series)
- 🔜 Notifications & Reminders
  - 🔜 New episodes
  - 🔜 Status changes

### v?.? (future)

- 🔜 Achievements & Leveling System
- 🔜 Configurable region, language, etc.
- 🔜 Ratings from IMDB

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
