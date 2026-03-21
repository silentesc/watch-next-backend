# Movie Release Dates Endpoint

### Endpoint

```
movie/{movie_id}/release_dates
```

### Path Params
| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ❌ | `movie_id` | `i32` | `1234` | |

### Example Response

> Keep in mind that everything except whats declared "never null" might be null or non existant (due to missing documentation on the TMDB API reference) to keep the endpoint stable (no random 500 errors)

```json
{
  "id": 2, // never null
  "results": [ // never null
    {
      "iso_3166_1": "DE", // never null
      "release_dates": [ // never null
        {
          "certification": "",
          "descriptors": [],
          "iso_639_1": "",
          "note": "Berlin International Film Festival",
          "release_date": "1989-02-16T00:00:00.000Z", // never null
          "type": 1 // never null
        },
        {
          "certification": "16",
          "descriptors": [],
          "iso_639_1": "",
          "note": "",
          "release_date": "1989-08-31T00:00:00.000Z", // never null
          "type": 3 // never null
        }
      ]
    }
  ]
}
```
