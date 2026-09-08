# TV Show Details Endpoint

### Endpoint

```
tv/{show_id}
```

### Path Params
| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ❌ | `show_id` | `i32` | `1234` | |

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ✅ | `language ` | `String` | `en-US` | Defaults to `en-US` |

### Example Response

> Keep in mind that everything except the id are declared to might be null or non existant (due to missing documentation on the TMDB API reference) to keep the endpoint stable (no random 500 errors)

> [!NOTE]
> `seasons`, `last_episode_to_air` and `next_episode_to_air` are not implemented yet

```json
{
  "id": 1399,
  "adult": false,
  "softcore": false,
  "in_production": false,
  "backdrop_path": "/zZqpAXxVSBtxV9qPBcscfXBcL2w.jpg",
  "poster_path": "/1XS1oqL89opfnbLl8WnZY1O1uJx.jpg",
  "genres": [
    {
      "id": 10765,
      "name": "Sci-Fi & Fantasy"
    },
    {
      "id": 18,
      "name": "Drama"
    },
    {
      "id": 10759,
      "name": "Action & Adventure"
    }
  ],
  "homepage": "https://www.hbo.com/game-of-thrones",
  "origin_country": [
    "US"
  ],
  "original_language": "en",
  "original_name": "Game of Thrones",
  "name": "Game of Thrones",
  "overview": "Seven noble families fight for control of the mythical land of Westeros. Friction between the houses leads to full-scale war. All while a very ancient evil awakens in the farthest north. Amidst the war, a neglected military order of misfits, the Night's Watch, is all that stands between the realms of men and icy horrors beyond.",
  "popularity": 241.0392,
  "production_companies": [
    {
      "id": 76043,
      "name": "Revolution Sun Studios",
      "origin_country": "US",
      "logo_path": "/9RO2vbQ67otPrBLXCaC8UMp3Qat.png"
    },
    {
      "id": 12525,
      "name": "Television 360",
      "origin_country": "",
      "logo_path": null
    },
    {
      "id": 5820,
      "name": "Generator Entertainment",
      "origin_country": "GB",
      "logo_path": null
    },
    {
      "id": 12526,
      "name": "Bighead Littlehead",
      "origin_country": "US",
      "logo_path": null
    },
    {
      "id": 286828,
      "name": "Grok! Television",
      "origin_country": "",
      "logo_path": null
    },
    {
      "id": 3268,
      "name": "HBO",
      "origin_country": "US",
      "logo_path": "/tuomPhY2UtuPTqqFnKMVHvSb724.png"
    }
  ],
  "production_countries": [
    {
      "name": "United Kingdom",
      "iso_3166_1": "GB"
    },
    {
      "name": "United States of America",
      "iso_3166_1": "US"
    }
  ],
  "networks": [
    {
      "id": 49,
      "name": "HBO",
      "origin_country": "US",
      "logo_path": "/tuomPhY2UtuPTqqFnKMVHvSb724.png"
    }
  ],
  "created_by": [
    {
      "adult": null,
      "gender": 2,
      "id": 9813,
      "known_for_department": null,
      "name": "David Benioff",
      "original_name": "David Benioff",
      "popularity": null,
      "profile_path": "/xvNN5huL0X8yJ7h3IZfGG4O2zBD.jpg",
      "credit_id": "5256c8c219c2956ff604858a",
      "department": null,
      "job": null
    },
    {
      "adult": null,
      "gender": 2,
      "id": 228068,
      "known_for_department": null,
      "name": "D. B. Weiss",
      "original_name": "D. B. Weiss",
      "popularity": null,
      "profile_path": "/6Wt006TIQoDSSnl0YaKihfn3w7K.jpg",
      "credit_id": "552e611e9251413fea000901",
      "department": null,
      "job": null
    }
  ],
  "first_air_date": "2011-04-17",
  "episode_run_time": [],
  "number_of_episodes": 73,
  "number_of_seasons": 8,
  "languages": [
    "en"
  ],
  "spoken_languages": [
    {
      "iso_639_1": "en",
      "english_name": "English",
      "name": "English"
    }
  ],
  "status": "Ended",
  "tagline": "Winter is coming.",
  "type": "Scripted",
  "vote_average": 8.47,
  "vote_count": 27688
}
```
