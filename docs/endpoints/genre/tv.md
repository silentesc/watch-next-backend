# Genre TV Endpoint

### Endpoint

```
/genre/tv/list
```

### Query Params

| Optional | Param | Type | Example | Description |
| --- | --- | --- | --- | --- |
| ✅ | `language` | `String` | `uk` | Defaults to `en` |

### Example Response

```json
{
  "genres": [
    {
      "id": 10759,
      "name": "Action & Adventure"
    },
    {
      "id": 16,
      "name": "Animation"
    },
    {
      "id": 35,
      "name": "Comedy"
    }
  ]
}
```
