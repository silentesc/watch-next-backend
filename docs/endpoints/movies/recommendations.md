# Movie Recommendations Endpoint

### Endpoint

```
/movie/{movie_id}/similar
```

### Path Params
| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ❌ | `movie_id` | `i32` | `1234` | |

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ✅ | `page` | `i32` | `1` | Defaults to `1` |
| ✅ | `language ` | `String` | `en-US` | Defaults to `en-US` |

### Example Response

> Keep in mind that everything except the id are declared to might be null or non existant (due to missing documentation on the TMDB API reference) to keep the endpoint stable (no random 500 errors)

```json
{
  "page": 1,
  "results": [
    {
      "adult": false,
      "backdrop_path": "/bmyugjdRl1aEOfBePwUpykIY3x3.jpg",
      "id": 12888,
      "title": "Belly",
      "original_title": "Belly",
      "overview": "Tommy Bundy and Sincere are best friends as well as infamous and ruthless criminals and shot-callers in the hood. Respected by many but feared by all.  As the police are closing in on them and new players are looking for a come up, will their reign last?",
      "poster_path": "/i4QhjdZXxdOpKOhc9kh4qqWjRWr.jpg",
      "media_type": "movie",
      "original_language": "en",
      "genre_ids": [
        80,
        18
      ],
      "popularity": 3.1632,
      "release_date": "1998-11-04",
      "video": false,
      "vote_average": 6.016,
      "vote_count": 156
    },
    {
      "adult": false,
      "backdrop_path": "/4kr8dUmxSxCHQnaI6hYtEz49OvR.jpg",
      "id": 22073,
      "title": "Hoodlum",
      "original_title": "Hoodlum",
      "overview": "In 1934, the second most lucrative business in New York City was running 'the numbers'. When Madam Queen—the powerful woman who runs the scam in Harlem—is arrested, Ellsworth 'Bumpy' Johnson takes over the business and must resist an invasion from a merciless mobster.",
      "poster_path": "/hZrwqVXLyxxsNJHVc6bJKSUDkXA.jpg",
      "media_type": "movie",
      "original_language": "en",
      "genre_ids": [
        80,
        18
      ],
      "popularity": 5.0173,
      "release_date": "1997-08-27",
      "video": false,
      "vote_average": 6.345,
      "vote_count": 229
    }
  ]
}
```
