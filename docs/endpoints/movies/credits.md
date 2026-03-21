# Movie Credits Endpoint

### Endpoint

```
movie/{movie_id}/credits
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
  "cast": [
    {
      "adult": false,
      "gender": 2,
      "id": 2037,
      "known_for_department": "Acting",
      "name": "Cillian Murphy",
      "original_name": "Cillian Murphy",
      "popularity": 8.5226,
      "profile_path": "/llkbyWKwpfowZ6C8peBjIV9jj99.jpg",
      "cast_id": 1,
      "character": "Tommy Shelby",
      "credit_id": "614705a485da120020bd381f",
      "order": 0
    },
    {
      "adult": false,
      "gender": 1,
      "id": 933238,
      "known_for_department": "Acting",
      "name": "Rebecca Ferguson",
      "original_name": "Rebecca Ferguson",
      "popularity": 9.8286,
      "profile_path": "/lJloTOheuQSirSLXNA3JHsrMNfH.jpg",
      "cast_id": 16,
      "character": "Kaulo / Zelda",
      "credit_id": "66a27781eac1fab882467caf",
      "order": 1
    }
  ],
  "crew": [
    {
      "adult": false,
      "gender": 0,
      "id": 1189235,
      "known_for_department": "Production",
      "name": "Andrew Warren",
      "original_name": "Andrew Warren",
      "popularity": 0.4564,
      "profile_path": null,
      "credit_id": "665f6ef0d0599a12e47d5bbf",
      "department": "Production",
      "job": "Executive Producer"
    },
    {
      "adult": false,
      "gender": 2,
      "id": 959442,
      "known_for_department": "Production",
      "name": "Guy Heeley",
      "original_name": "Guy Heeley",
      "popularity": 0.832,
      "profile_path": null,
      "credit_id": "665f6ec957cda84e6e3845d4",
      "department": "Production",
      "job": "Producer"
    }
  ]
}
```
