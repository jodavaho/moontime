curl -X GET  'localhost:3000/s/cadre/solartime'
curl -X GET  'localhost:3000/s/cadre/sun/azel'
curl -X GET  'localhost:3000/s/et?t=2024-02-01T19%3A01%3A16Z'
curl -X GET  'localhost:3000/s/et?t=2024-02-01T19%3A01%3A16Z&f=json'
curl -X GET  'localhost:3000/s/et?t=2024-02-24T12:05:00.00+00:00&f=json' -i
curl -X POST 'localhost:3000/s/et?t=2024-02-24T12:05:00.00+00:00&f=txt' -i
curl -X POST 'localhost:3000/s/cadre/sun/azel' -d '{"t"="2024-02-24T12:05:00.00+00:00", "f":"json", "pos"={ "lat":0.0, "lon":0.0, "alt":0.0 } }' -i
