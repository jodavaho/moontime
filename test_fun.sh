curl         'http://localhost:3000/s/cadre/solartime'
curl         'http://localhost:3000/s/cadre/sun/azel'
curl -X GET  'http://localhost:3000/s/et?t=2024-02-24T12:05:00.00+00:00&f=json' -i
curl -X POST 'http://localhost:3000/s/et?t=2024-02-24T12:05:00.00+00:00&f=json' -i
curl -X POST 'http://localhost:3000/s/cadre/sun/azel' -d '{"t"="2024-02-24T12:05:00.00+00:00", "f":"json", "pos"={ "lat":0.0, "lon":0.0, "alt":0.0 } }' -i
