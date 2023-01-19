# NextJS networking test

## The dream

Running both frontend and backend in the same docker compose file, and frontend being able to access the backend through docker-composes internal networks.

Since both containers are on the same network after the build phase this shouldn't be a problem.

This should enable Next.js to build using SSG and ISG.

## The reality

```
docker compose up -d backend
docker compose build frontend
```

This will fail because the frontend container does not have access to the backend container during build.