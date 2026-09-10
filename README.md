# Watch Next

## Endpoints

#### Endpoints are based on the [TMDB API](https://developer.themoviedb.org/reference).

| Category | Endpoints |
| --- | --- |
| Discover | <ul><li>[Movie](docs/endpoints/discover/movie.md)</li><li>[TV](docs/endpoints/discover/tv.md)</li></ul> |
| Trending | <ul><li>[Movies](docs/endpoints/trending/movies.md)</li><li>[TV](docs/endpoints/trending/tv.md)</li></ul> |
| Search | <ul><li>[Movie](docs/endpoints/search/movie.md)</li><li>[TV](docs/endpoints/search/tv.md)</li><li>[Collection](docs/endpoints/search/collection.md)</li></ul> |
| Genre | <ul><li>[Movie](docs/endpoints/genre/movie.md)</li><li>[TV](docs/endpoints/genre/tv.md)</li></ul> |
| Configuration | <ul><li>[Languages](docs/endpoints/configuration/languages.md)</li></ul> |
| Movies | <ul><li>[Details](docs/endpoints/movies/details.md)</li><li>[Release Dates](docs/endpoints/movies/release_dates.md)</li><li>[Credits](docs/endpoints/movies/credits.md)</li><li>[Videos](docs/endpoints/movies/videos.md)</li><li>[Recommendations](docs/endpoints/movies/recommendations.md)</li><li>[Similar](docs/endpoints/movies/similar.md)</li></ul> |
| TV Series | <ul><li>[Details](docs/endpoints/tv_series/details.md)</li><li>[Videos](docs/endpoints/tv_series/videos.md)</li><li>[Recommendations](docs/endpoints/tv_series/recommendations.md)</li></ul> |
| TV Seasons | <ul><li>[Details](docs/endpoints/tv_seasons/details.md)</li></ul> |
| Collections | <ul><li>[Details](docs/endpoints/collections/details.md)</li></ul> |

## Roadmap

### **v1.0** (current)

- **Planned features**
  - 🔜 Lists (e.g. Plan to Watch, Watching, Completed)
  - 🔜 Custom Tags per movie/series (e.g. must-watch)
  - 🔜 Auto updating data in db
  - 🔜 Caching
- **Planned endpoints**
  - ✅ Discover
    - ✅ Movie
    - ✅ TV Series
  - 🔜 Trending
    - 🔜 All
    - ✅ Movie
    - ✅ TV Series
  - 🔜 Search
    - ✅ Collection
    - ✅ Movie
    - 🔜 Multi
    - 🔜 Person
    - ✅ TV Series
  - ✅ Genres
    - ✅ Movies
    - ✅ TV Series
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
  - 🔜 TV Series
    - ✅ Details
    - ✅ Videos
    - ✅ Recommendations
    - 🔜 Similar
  - ✅ TV Seasons
    - ✅ Details

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
- 🔜 Refresh token so sessions don't expire after 7 days

## Quick Start

### Database

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
    restart: unless-stopped
```

### Watch Next Backend

#### .env file
```dotenv
# TRACE, DEBUG, INFO, WARN, ERROR
LOG_LEVEL="INFO"

DATABASE_URL="postgres://my_postgres_user:strong_password@localhost/name_of_db"

SERVE_ADDR="0.0.0.0:3000"
CORS_ALLOWED_ORIGINS="http://localhost:5173,https://api.watch-next.mydomain.com"

TMDB_API_KEY="abcdefghijklmnopqrstuvwxyz"

# Must be at least 64 characters long
# Changing key invalidates all active sessions, causing users to have to login again
# Leaving it empty generates a secure, random, different one every startup
COOKIE_KEY="long_secret_key"
```

#### Run app
```bash
cargo run
```
