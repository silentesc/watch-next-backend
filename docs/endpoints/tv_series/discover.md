# Discover TV Series Endpoint

### Endpoint

```
/discover/tv
```

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ✅ | `page` | `i32` | `1` | Defaults to `1` |
| ✅ | `first_air_date.gte` | `String` | `2025-01-20` | |
| ✅ | `first_air_date.lte` | `String` | `2026-01-20` | |
| ✅ | `sort_by` | `String` | See below table | Defaults to `popularity.desc` |
| ✅ | `vote_average.gte` | `f32` | `6.5` | |
| ✅ | `vote_average.lte` | `f32` | `9.5` | |
| ✅ | `vote_count.gte` | `f32` | `6500` | |
| ✅ | `vote_count.lte` | `f32` | `10000` | |
| ✅ | `with_genres ` | `String` | `Action,Animation` | comma (`,`) for `AND`, pipe (`\|`) for `OR` |
| ✅ | `without_genres ` | `String` | `Action,Animation` | comma (`,`) for `AND`, pipe (`\|`) for `OR` |
| ✅ | `with_origin_country ` | `String` | `uk` | |
| ✅ | `with_original_language ` | `String` | `uk` | |
| ✅ | `with_runtime.gte` | `i32` | `93` | In minutes |
| ✅ | `with_runtime.lte` | `i32` | `120` | In minutes |

#### Sort by Values

- `original_name.asc`
- `original_name.desc`
- `popularity.asc`
- `popularity.desc`
- `first_air_date.asc`
- `first_air_date.desc`
- `name.asc`
- `name.desc`
- `vote_average.asc`
- `vote_average.desc`
- `vote_count.asc`
- `vote_count.desc`

### Example Response

> Keep in mind that everything except the id are declared to might be null or non existant (due to missing documentation on the TMDB API reference) to keep the endpoint stable (no random 500 errors)

```json
{
  "total_results": 1110455,
  "total_pages": 55523,
  "page": 1,
  "results": [
    {
      "adult": false,
      "backdrop_path": "/pF0qkRsrHkdYadPWY9AMeFZfcwk.jpg",
      "genre_ids": [
        10759,
        80
      ],
      "id": 108978,
      "origin_country": [
        "US"
      ],
      "original_language": "en",
      "original_name": "Reacher",
      "overview": "Description of the series.",
      "popularity": 584.2322,
      "poster_path": "/f1VCQIG2iCyOookdgOzwtUpwWC0.jpg",
      "first_air_date": "2022-02-03",
      "softcore": false,
      "name": "Reacher",
      "vote_average": 8.108,
      "vote_count": 3234
    }
  ]
}
```
