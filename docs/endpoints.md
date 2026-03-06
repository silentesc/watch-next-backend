# Endpoints

## Discover

```
/discover/movies
```

### Path Params

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
