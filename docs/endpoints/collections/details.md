# Collection Details Endpoint

### Endpoint

```
collection/{collection_id}
```

### Path Params
| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ❌ | `collection_id` | `i32` | `1234` | |

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ✅ | `language ` | `String` | `en-US` | Defaults to `en-US` |

### Example Response

> Keep in mind that everything except the id are declared to might be null or non existant (due to missing documentation on the TMDB API reference) to keep the endpoint stable (no random 500 errors)

```json
{
  "id": 10,
  "name": "Star Wars Collection",
  "original_language": "en",
  "original_name": "Star Wars Collection",
  "overview": "An epic space-opera theatrical film series, which depicts the adventures of various characters \"a long time ago in a galaxy far, far away….\"",
  "poster_path": "/22dj38IckjzEEUZwN1tPU5VJ1qq.jpg",
  "backdrop_path": "/4z9ijhgEthfRHShoOvMaBlpciXS.jpg",
  "parts": [
    {
      "adult": false,
      "backdrop_path": "/2w4xG178RpB4MDAIfTkqAuSJzec.jpg",
      "id": 11,
      "name": "Star Wars",
      "original_name": "Star Wars",
      "overview": "Princess Leia is captured and held hostage by the evil Imperial forces in their effort to take over the galactic Empire. Venturesome Luke Skywalker and dashing captain Han Solo team together with the loveable robot duo R2-D2 and C-3PO to rescue the beautiful princess and restore peace and justice in the Empire.",
      "poster_path": "/6FfCtAuVAW8XJjZ7eWeLibRLWTw.jpg",
      "media_type": "movie",
      "original_language": "en",
      "genre_ids": [
        12,
        28,
        878
      ],
      "popularity": 15.8557,
      "release_date": "1977-05-25",
      "video": false,
      "vote_average": 8.205,
      "vote_count": 21522
    },
    {
      "adult": false,
      "backdrop_path": "/dMZxEdrWIzUmUoOz2zvmFuutbj7.jpg",
      "id": 1891,
      "name": "The Empire Strikes Back",
      "original_name": "The Empire Strikes Back",
      "overview": "The epic saga continues as Luke Skywalker, in hopes of defeating the evil Galactic Empire, learns the ways of the Jedi from aging master Yoda. But Darth Vader is more determined than ever to capture Luke. Meanwhile, rebel leader Princess Leia, cocky Han Solo, Chewbacca, and droids C-3PO and R2-D2 are thrown into various stages of capture, betrayal and despair.",
      "poster_path": "/nNAeTmF4CtdSgMDplXTDPOpYzsX.jpg",
      "media_type": "movie",
      "original_language": "en",
      "genre_ids": [
        12,
        28,
        878
      ],
      "popularity": 6.7352,
      "release_date": "1980-05-20",
      "video": false,
      "vote_average": 8.395,
      "vote_count": 17713
    }
  ]
}
```
