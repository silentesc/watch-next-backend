# TV Series Recommendations Endpoint

### Endpoint

```
/tv/{series_id}/recommendations
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
  "total_results": 497,
  "total_pages": 25,
  "page": 1,
  "results": [
    {
      "adult": false,
      "backdrop_path": "/577eXC8wFQT0eUrJcgznSiFPRmk.jpg",
      "poster_path": "/7V0Ebks0GgpKvQ7QbLAIdX5dos4.jpg",
      "genre_ids": [
        10765,
        18,
        10759
      ],
      "id": 94997,
      "original_language": "en",
      "original_name": "House of the Dragon",
      "name": "House of the Dragon",
      "overview": "The Targaryen dynasty is at the absolute apex of its power, with more than 15 dragons under their yoke. Most empires crumble from such heights. In the case of the Targaryens, their slow fall begins when King Viserys breaks with a century of tradition by naming his daughter Rhaenyra heir to the Iron Throne. But when Viserys later fathers a son, the court is shocked when Rhaenyra retains her status as his heir, and seeds of division sow friction across the realm.",
      "popularity": 130.791,
      "first_air_date": "2022-08-21",
      "vote_average": 8.382,
      "vote_count": 7102
    }
  ]
}
```
