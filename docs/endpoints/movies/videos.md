# Movie Videos Endpoint

### Endpoint

```
movie/{movie_id}/videos
```

### Path Params
| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ❌ | `movie_id` | `i32` | `1234` | |

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ✅ | `language ` | `String` | `en-US` | Defaults to `en-US` |

### Example Response

> Keep in mind that everything except the ids are declared to might be null or non existant (due to missing documentation on the TMDB API reference) to keep the endpoint stable (no random 500 errors)

```json
{
  "id": 875828,
  "results": [
    {
      "iso_639_1": "en",
      "iso_3166_1": "US",
      "name": "Sneak Peek - Meet Duke Shelby",
      "key": "hd8-Y5FnPJ8",
      "site": "YouTube",
      "size": 1080,
      "type": "Clip",
      "official": true,
      "published_at": "2026-03-05T22:00:00.000Z",
      "id": "69ab6b1212d7ec3c2d58b156"
    },
    {
      "iso_639_1": "en",
      "iso_3166_1": "US",
      "name": "Official Trailer",
      "key": "lcvUGs3xaDM",
      "site": "YouTube",
      "size": 1080,
      "type": "Trailer",
      "official": true,
      "published_at": "2026-02-19T14:00:01.000Z",
      "id": "69971fa87d0d5a59881ae9b5"
    },
    {
      "iso_639_1": "en",
      "iso_3166_1": "US",
      "name": "Official Teaser",
      "key": "_Dfc89TY-aA",
      "site": "YouTube",
      "size": 1080,
      "type": "Teaser",
      "official": true,
      "published_at": "2025-12-24T14:00:00.000Z",
      "id": "694bf79c6a1ffd7db0b1ba7c"
    }
  ]
}
```
