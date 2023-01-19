# NextJS networking test

```
docker compose up -d backend
docker compose build frontend
```

This will fail because the frontend container does not have access to the backend container during build.