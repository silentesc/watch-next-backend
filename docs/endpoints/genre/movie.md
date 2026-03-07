# Genre Movie Endpoint

### Endpoint

```
/genre/movie/list
```

### Path Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ✅ | `language ` | `String` | `uk` | Defaults to `en` |

### Example Response

```json
{
  "genres": [
    {
      "id": 28,
      "name": "Action"
    },
    {
      "id": 12,
      "name": "Adventure"
    },
    {
      "id": 16,
      "name": "Animation"
    }
  ]
}
```
