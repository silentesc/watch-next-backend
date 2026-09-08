# Trending TV Shows Endpoint

### Endpoint

```
/trending/tv/{time_window}
```

### Path Params
| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ❌ | `time_window` | `String` | `day` | `day` or `week` |

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ✅ | `language` | `String` | `en-US` | Defaults to `en-US` |

### Example Response

> Keep in mind that everything except the id are declared to might be null or non existant (due to missing documentation on the TMDB API reference) to keep the endpoint stable (no random 500 errors)

```json
{
  "page": 1,
  "results": [
    {
      "adult": false,
      "backdrop_path": "/wJjnJbVUwPz0GADAgpFt9nWtzUu.jpg",
      "id": 95350,
      "name": "Lanterns",
      "original_name": "Lanterns",
      "overview": "Two intergalactic cops, new recruit John Stewart and Lantern legend Hal Jordan, are drawn into a dark, Earth-based mystery as they investigate a murder in the American heartland.",
      "poster_path": "/gpC7h43xPMEV3goYMQShfJbTtLq.jpg",
      "media_type": "tv",
      "original_language": "en",
      "genre_ids": [
        18,
        9648,
        10765
      ],
      "popularity": 277.2042,
      "first_air_date": "2026-08-16",
      "softcore": false,
      "vote_average": 8.3,
      "vote_count": 288,
      "origin_country": [
        "US"
      ]
    },
    {
      "adult": false,
      "backdrop_path": "/4XccmjsOmQZw8S2iW1wvlvmb5v1.jpg",
      "id": 125988,
      "name": "Silo",
      "original_name": "Silo",
      "overview": "In a ruined and toxic future, thousands live in a giant silo deep underground. After its sheriff breaks a cardinal rule and residents die mysteriously, engineer Juliette starts to uncover shocking secrets and the truth about the silo.",
      "poster_path": "/gMYZZvnkVNTqSVnVCphWbPXwWwb.jpg",
      "media_type": "tv",
      "original_language": "en",
      "genre_ids": [
        10765,
        18
      ],
      "popularity": 346.844,
      "first_air_date": "2023-05-04",
      "softcore": false,
      "vote_average": 8.2,
      "vote_count": 2610,
      "origin_country": [
        "US"
      ]
    }
  ],
  "total_pages": 500,
  "total_results": 10000
}
```
