# Discover Movie Endpoint

### Endpoint

```
/discover/movie
```

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ✅ | `page` | `i32` | `1` | Defaults to `1` |
| ✅ | `primary_release_date.gte` | `String` | `2025-01-20` | |
| ✅ | `primary_release_date.lte` | `String` | `2026-01-20` | |
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

- `original_title.asc`
- `original_title.desc`
- `popularity.asc`
- `popularity.desc`
- `revenue.asc`
- `revenue.desc`
- `primary_release_date.asc`
- `primary_release_date.desc`
- `title.asc`
- `title.desc`
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
      "backdrop_path": "/nHxWyy18SvAZ8jJeemtS8k1UNjM.jpg",
      "poster_path": "/buPFnHZ3xQy6vZEHxbHgL1Pc6CR.jpg",
      "genre_ids": [
        28,
        80,
        53
      ],
      "id": 1290821,
      "original_language": "en",
      "original_title": "Shelter",
      "title": "Shelter",
      "overview": "Description of the movie.",
      "popularity": 372.8761,
      "release_date": "2026-01-28",
      "video": false,
      "vote_average": 6.783,
      "vote_count": 224
    }
  ]
}
```
