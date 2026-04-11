# Trending Movie Endpoint

### Endpoint

```
/trending/movie/{time_window}
```

### Path Params
| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ❌ | `time_window` | `String` | `day` | `day` or `week` |

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ✅ | `language` | `String` | `en-US` | Defaults to `en-US` |

### Example Response

> Keep in mind that everything except the id are declared to might be null or non existant (due to missing documentation on the TMDB API reference) to keep the endpoint stable (no random 500 errors)

```json
{
  "page": 1,
  "results": [
    {
      "adult": false,
      "backdrop_path": "/2RrLuIfIzGWWIH8IAEo6o0IYHmx.jpg",
      "id": 1327819,
      "title": "Hoppers",
      "original_title": "Hoppers",
      "overview": "Scientists have discovered how to 'hop' human consciousness into lifelike robotic animals, allowing people to communicate with animals as animals. Animal lover Mabel seizes an opportunity to use the technology, uncovering mysteries within the animal world beyond anything she could have imagined.",
      "poster_path": "/xjtWQ2CL1mpmMNwuU5HeS4Iuwuu.jpg",
      "original_language": "en",
      "genre_ids": [
        16,
        10751,
        878,
        35
      ],
      "popularity": 134.1029,
      "release_date": "2026-03-04",
      "video": false,
      "vote_average": 7.7,
      "vote_count": 234
    },
    {
      "adult": false,
      "backdrop_path": "/8Tfys3mDZVp4tNoH2ktm06a0Tau.jpg",
      "id": 687163,
      "title": "Project Hail Mary",
      "original_title": "Project Hail Mary",
      "overview": "Science teacher Ryland Grace wakes up on a spaceship light years from home with no recollection of who he is or how he got there. As his memory returns, he begins to uncover his mission: solve the riddle of the mysterious substance causing the sun to die out. He must call on his scientific knowledge and unorthodox ideas to save everything on Earth from extinction… but an unexpected friendship means he may not have to do it alone.",
      "poster_path": "/huVzcVrlK8aiLd240ienleODvWl.jpg",
      "original_language": "en",
      "genre_ids": [
        878,
        12,
        9648
      ],
      "popularity": 210.0281,
      "release_date": "2026-03-15",
      "video": false,
      "vote_average": 8.287,
      "vote_count": 178
    }
  ],
  "total_pages": 500,
  "total_results": 10000
}
```
