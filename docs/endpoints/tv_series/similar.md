# Similar TV Series Endpoint

### Endpoint

```
/tv/{series_id}/similar
```

### Path Params
| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ❌ | `series_id` | `i32` | `1234` | |

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ✅ | `page` | `i32` | `1` | Defaults to `1` |
| ✅ | `language ` | `String` | `en-US` | Defaults to `en-US` |

### Example Response

> Keep in mind that everything except the id are declared to might be null or non existant (due to missing documentation on the TMDB API reference) to keep the endpoint stable (no random 500 errors)

```json
{
  "total_results": 62524,
  "total_pages": 3127,
  "page": 1,
  "results": [
    {
      "adult": false,
      "backdrop_path": "/5mcc7J07f680Q88xeLClf5SIZtt.jpg",
      "poster_path": "/9zrvktj9JYQwQxZD4lAySfcYlmI.jpg",
      "genre_ids": [
        18,
        10765
      ],
      "id": 207333,
      "original_language": "es",
      "original_name": "Cien años de soledad",
      "name": "One Hundred Years of Solitude",
      "overview": "In the mythical town Macondo, seven generations of the Buendía family navigate love, oblivion and the inescapability of their past — and their fate.",
      "popularity": 28.6502,
      "first_air_date": "2024-12-11",
      "vote_average": 8.057,
      "vote_count": 261
    }
  ]
}
```
