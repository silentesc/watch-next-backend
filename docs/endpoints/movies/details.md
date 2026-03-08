# Discover Movie Endpoint

### Endpoint

```
movie/{movie_id}
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

```json
{
  "adult": false,
  "backdrop_path": "/l94l89eMmFKh7na2a1u5q67VgNx.jpg",
  "belongs_to_collection": { // can be null
    "id": 1382526,
    "name": "Aki Kaurismäki's Proletariat Trilogy",
    "poster_path": "/bUrReoZFLGti6ehkBW0xw8f12MT.jpg", // can be null
    "backdrop_path": "/zAUItK1Nr473DIe8gWMsZ0DMR7L.jpg" // can be null
  },
  "budget": 0,
  "genres": [
    {
      "id": 35,
      "name": "Comedy"
    },
    {
      "id": 18,
      "name": "Drama"
    },
    {
      "id": 10749,
      "name": "Romance"
    }
  ],
  "homepage": "",
  "id": 3,
  "imdb_id": "tt0092149", // can be null
  "origin_country": [ // can be null
    "FI"
  ],
  "original_language": "fi",
  "original_title": "Varjoja paratiisissa",
  "overview": "Nikander, a rubbish collector and would-be entrepreneur, finds his plans for success dashed when his business associate dies. One evening, he meets Ilona, a down-on-her-luck cashier, in a local supermarket. Falteringly, a bond begins to develop between them.",
  "popularity": 1.1736,
  "poster_path": "/nj01hspawPof0mJmlgfjuLyJuRN.jpg", // can be null
  "production_companies": [
    {
      "id": 2303,
      "logo_path": null, // can be null
      "name": "Villealfa Filmproductions",
      "origin_country": "FI"
    }
  ],
  "production_countries": [
    {
      "iso_3166_1": "FI",
      "name": "Finland"
    }
  ],
  "release_date": "1986-10-17",
  "revenue": 0,
  "runtime": 74,
  "spoken_languages": [
    {
      "english_name": "Swedish",
      "iso_639_1": "sv",
      "name": "svenska"
    },
    {
      "english_name": "Finnish",
      "iso_639_1": "fi",
      "name": "suomi"
    },
    {
      "english_name": "English",
      "iso_639_1": "en",
      "name": "English"
    }
  ],
  "status": "Released",
  "tagline": "",
  "title": "Shadows in Paradise",
  "video": false,
  "vote_average": 7.262,
  "vote_count": 434
}
```
