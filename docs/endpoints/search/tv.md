# Search TV Shows Endpoint

### Endpoint

```
/search/tv
```

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ❌ | `query` | `String` | `Star Wars` | |
| ✅ | `page` | `i32` | `1` | Defaults to `1` |
| ✅ | `include_adult` | `bool` | `true` | Defaults to `false` |
| ✅ | `language` | `String` | `de-DE` | Defaults to `en-US` |
| ✅ | `first_air_date_year` | `i32` | `2025` | The year the show was released |
| ✅ | `year` | `i32` | `2025` | Broader year spectrum, not tied to release year. (e.g. production year) |

### Example Response

> Keep in mind that everything except the id are declared to might be null or non existant (due to missing documentation on the TMDB API reference) to keep the endpoint stable (no random 500 errors)

```json
{
  "page": 1,
  "results": [
    {
      "adult": false,
      "backdrop_path": "/m6eRgkR1KC6Mr6gKx6gKCzSn6vD.jpg",
      "genre_ids": [
        10759,
        10765,
        16
      ],
      "id": 4194,
      "origin_country": [
        "US"
      ],
      "original_language": "en",
      "original_name": "Star Wars: The Clone Wars",
      "overview": "Yoda, Obi-Wan Kenobi, Anakin Skywalker, Mace Windu and other Jedi Knights lead the Grand Army of the Republic against the droid army of the Separatists.",
      "popularity": 84.1964,
      "poster_path": "/e1nWfnnCVqxS2LeTO3dwGyAsG2V.jpg",
      "first_air_date": "2008-10-03",
      "softcore": false,
      "name": "Star Wars: The Clone Wars",
      "vote_average": 8.465,
      "vote_count": 2421
    }
  ],
  "total_pages": 4,
  "total_results": 69
}
```
