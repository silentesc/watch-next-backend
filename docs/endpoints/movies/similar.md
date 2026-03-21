# Similar Movies Endpoint

### Endpoint

```
/movie/{movie_id}/similar
```

### Path Params
| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ❌ | `movie_id` | `i32` | `1234` | |

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ✅ | `page` | `i32` | `1` | Defaults to `1` |
| ✅ | `language ` | `String` | `en-US` | Defaults to `en-US` |

### Example Response

> Keep in mind that everything except the id are declared to might be null or non existant (due to missing documentation on the TMDB API reference) to keep the endpoint stable (no random 500 errors)

```json
{
  "page": 1,
  "results": [
    {
      "adult": false,
      "backdrop_path": "/kG93zIRD0nnD4MPmNfGRIf8AFxQ.jpg",
      "genre_ids": [
        18,
        36,
        10752
      ],
      "id": 16914,
      "original_language": "en",
      "original_title": "The Cruel Sea",
      "overview": "At the start of World War II, Cmdr. Ericson is assigned to convoy escort HMS Compass Rose with inexperienced officers and men just out of training. The winter seas make life miserable enough, but the men must also harden themselves to rescuing survivors of U-Boat attacks, while seldom able to strike back. Traumatic events afloat and ashore create a warm bond between the skipper and his first officer",
      "popularity": 1.3832,
      "poster_path": "/kmMGoeLx7rntdbqKZFp6bmT8ZCW.jpg",
      "release_date": "1953-02-24",
      "title": "The Cruel Sea",
      "video": false,
      "vote_average": 6.885,
      "vote_count": 78
    },
    {
      "adult": false,
      "backdrop_path": null,
      "genre_ids": [
        80,
        28,
        35
      ],
      "id": 1319232,
      "original_language": "pl",
      "original_title": "Łańcuchy pokarmowe terenów zalesionych",
      "overview": "Shady interests, scores, revenge - away from the hustle and bustle of the city you will find its brutal finale in the quiet heart of a pine forest. Each of the heroes of this bloody confrontation soon discovers his place in the food chain.",
      "popularity": 0.1678,
      "poster_path": "/eORCpQKPOsK9F9H0LJDGM2UjNH6.jpg",
      "release_date": "2016-09-15",
      "title": "Food Chain Complexity in Afforested Grounds",
      "video": false,
      "vote_average": 0,
      "vote_count": 0
    }
  ],
  "total_pages": 15594,
  "total_results": 311862
}
```
