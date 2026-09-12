# TV Season Details Endpoint

### Endpoint

```
tv/{series_id}/season/{season_number}
```

### Path Params
| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ❌ | `series_id` | `i32` | `1234` | |
| ❌ | `season_number` | `i32` | `1234` | |

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ✅ | `language ` | `String` | `en-US` | Defaults to `en-US` |

### Example Response

> Keep in mind that everything except the id are declared to might be null or non existant (due to missing documentation on the TMDB API reference) to keep the endpoint stable (no random 500 errors)

```json
{
  "air_date": "2011-04-17",
  "episodes": [
    {
      "air_date": "2011-04-17",
      "episode_number": 1,
      "episode_type": "standard",
      "id": 63056,
      "name": "Winter Is Coming",
      "overview": "Jon Arryn, the Hand of the King, is dead. King Robert Baratheon plans to ask his oldest friend, Eddard Stark, to take Jon's place. Across the sea, Viserys Targaryen plans to wed his sister to a nomadic warlord in exchange for an army.",
      "production_code": "101",
      "runtime": 62,
      "season_number": 1,
      "show_id": 1399,
      "still_path": "/o4IX9Mm0kpLITVANJMx7inyEUaY.jpg",
      "vote_average": 8.161,
      "vote_count": 528,
      "crew": [
        {
          "adult": false,
          "gender": 2,
          "id": 9813,
          "known_for_department": "Writing",
          "name": "David Benioff",
          "original_name": "David Benioff",
          "popularity": 1.553,
          "profile_path": "/xvNN5huL0X8yJ7h3IZfGG4O2zBD.jpg",
          "credit_id": "5256c8a019c2956ff6046e2b",
          "department": "Writing",
          "job": "Writer"
        }
      ],
      "guest_stars": [
        {
          "character": "Benjen Stark",
          "credit_id": "5256c8b919c2956ff604836a",
          "order": 61,
          "adult": false,
          "gender": 2,
          "id": 119783,
          "known_for_department": "Acting",
          "name": "Joseph Mawle",
          "original_name": "Joseph Mawle",
          "popularity": 0.9431,
          "profile_path": "/1Ocb9v3h54beGVoJMm4w50UQhLf.jpg"
        }
      ]
    }
  ],
  "name": "Season 1",
  "networks": [
    {
      "id": 49,
      "name": "HBO",
      "origin_country": "US",
      "logo_path": "/tuomPhY2UtuPTqqFnKMVHvSb724.png"
    }
  ],
  "overview": "Trouble is brewing in the Seven Kingdoms of Westeros. For the driven inhabitants of this visionary world, control of Westeros' Iron Throne holds the lure of great power. But in a land where the seasons can last a lifetime, winter is coming...and beyond the Great Wall that protects them, an ancient evil has returned. In Season One, the story centers on three primary areas: the Stark and the Lannister families, whose designs on controlling the throne threaten a tenuous peace; the dragon princess Daenerys, heir to the former dynasty, who waits just over the Narrow Sea with her malevolent brother Viserys; and the Great Wall--a massive barrier of ice where a forgotten danger is stirring.",
  "id": 3624,
  "poster_path": "/wgfKiqzuMrFIkU1M68DDDY8kGC1.jpg",
  "season_number": 1,
  "vote_average": 8.4
}
```
