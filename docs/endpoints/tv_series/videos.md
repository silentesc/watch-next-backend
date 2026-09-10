# TV Series Videos Endpoint

### Endpoint

```
tv/{series_id}/videos
```

### Path Params
| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ❌ | `series_id` | `i32` | `1234` | |

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ✅ | `language ` | `String` | `en-US` | Defaults to `en-US` |

### Example Response

> Keep in mind that everything except the ids are declared to might be null or non existant (due to missing documentation on the TMDB API reference) to keep the endpoint stable (no random 500 errors)

```json
{
  "id": 1399,
  "results": [
    {
      "iso_639_1": "en",
      "iso_3166_1": "US",
      "key": "KPLWWIOCOOQ",
      "site": "YouTube",
      "size": 1080,
      "type": "Trailer",
      "official": true,
      "published_at": "2021-04-05T16:00:07.000Z",
      "id": "64ec59a7e894a60101224a01"
    }
  ]
}
```
