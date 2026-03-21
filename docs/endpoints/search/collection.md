# Search Collection Endpoint

### Endpoint

```
/search/collection
```

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ❌ | `query` | `String` | `Star Wars` | |
| ✅ | `page` | `i32` | `1` | Defaults to `1` |
| ✅ | `include_adult` | `bool` | `true` | Defaults to `false` |
| ✅ | `language` | `String` | `de-DE` | Defaults to `en-US` |
| ✅ | `region` | `String` | `US` | `US` |

### Example Response

> Keep in mind that everything except the id are declared to might be null or non existant (due to missing documentation on the TMDB API reference) to keep the endpoint stable (no random 500 errors)

```json
{
  "page": 1,
  "results": [
    {
      "adult": false,
      "backdrop_path": "/1WDssJDYInLA4Avg45lgy3WM6Ly.jpg",
      "id": 1084247,
      "name": "Zootopia Collection",
      "original_language": "en",
      "original_name": "Zootopia Collection",
      "overview": "Zootopia is a bustling modern metropolis inhabited solely by anthropomorphic mammals. Predators and prey have long agreed to live together in peace and equality.  Ambitious rabbit Judy Hopps becomes the city’s first bunny police officer and crosses paths with sly fox Nick Wilde. From initial distrust grows a deep friendship and partnership.  Together, the unlikely duo solves challenging cases, navigates Zootopia’s diverse districts—from icy Tundratown to the humid Rainforest District—and fights prejudice. They prove that no matter how big or small, predator or prey, anyone can achieve their dreams in Zootopia.  Each film tells a standalone adventure, united by the evolving bond between Judy and Nick and the question of how a society functions when ancient instincts meet modern values.",
      "poster_path": "/9Cm0kU0TAcJ6MoWYKWOl2LzN70I.jpg"
    }
  ],
  "total_pages": 1,
  "total_results": 1
}
```
