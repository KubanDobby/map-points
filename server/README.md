# map-points-api

Создание точки

```bash
curl -X POST http://localhost:3000/points \
-H "Content-Type: application/json" \
-d '{
  "lng": 30.5,
  "lat": 50.4,
  "camp": "friend"
}'
```

Получение всех точек

```bash
curl http://localhost:3000/points
```