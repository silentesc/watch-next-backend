# Search Movie Endpoint

### Endpoint

```
/search/movie
```

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ❌ | `query` | `String` | `Star Wars` | |
| ✅ | `page` | `i32` | `1` | Defaults to `1` |
| ✅ | `include_adult` | `bool` | `true` | Defaults to `false` |
| ✅ | `language` | `String` | `de-DE` | Defaults to `en-US` |
| ✅ | `primary_release_year` | `String` | `2025` | The year the movie was released |
| ✅ | `region` | `String` | `US` | `US` |
| ✅ | `year` | `String` | `2025` | Broader year spectrum, not tied to release year. (e.g. production year) |

### Example Response

```json
{
  "page": 1,
  "total_pages": 10,
  "total_results": 192,
  "results": [
    {
      "adult": false,
      "backdrop_path": "/2w4xG178RpB4MDAIfTkqAuSJzec.jpg", // can be null
      "poster_path": "/6FfCtAuVAW8XJjZ7eWeLibRLWTw.jpg",  // can be null
      "genre_ids": [
        12,
        28,
        878
      ],
      "id": 11,
      "original_language": "en",
      "original_title": "Star Wars",
      "overview": "Princess Leia is captured and held hostage by the evil Imperial forces in their effort to take over the galactic Empire. Venturesome Luke Skywalker and dashing captain Han Solo team together with the loveable robot duo R2-D2 and C-3PO to rescue the beautiful princess and restore peace and justice in the Empire.",
      "popularity": 16.6462,
      "release_date": "1977-05-25",
      "title": "Star Wars",
      "video": false,
      "vote_average": 8.2,
      "vote_count": 22008
    }
  ]
}
```
