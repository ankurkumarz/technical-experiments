---
title: Backstage
---

# Backstage
- [Backstage](https://backstage.io/), developed by Spotify, is an open platform for building developer portals.
- You can centralize software catalog, microservices catalog, infrastructure tooling, documentation (ADRs), etc. in a single place.
- [Click here](https://engineering.atspotify.com/2020/04/how-we-use-backstage-at-spotify/) to read the Spotify blog post providing overview.
- GitHub: https://github.com/backstage/backstage

Demo

- **Online Demo**: https://demo.backstage.io


# Local setup
- Guide: https://backstage.io/docs/getting-started/
- Crate a new backstage app using npx:
```
npx @backstage/create-app
yarn dev
```
- Local URL: http://localhost:3000
- Setting up database:
```
brew install postgresql
brew services restart postgresql@14 or /usr/local/opt/postgresql@14/bin/postgres -D /usr/local/var/postgresql@14

#Login in local postgresql
psql postgres -- to check postgres connection
CREATE USER backstage WITH PASSWORD 'secret' CREATEDB LOGIN
```
- Update configuration to use database:
```
yarn add --cwd packages/backend pg
```
- Update app-config.yaml:
```
 database:
    #client: better-sqlite3
    #connection: ':memory:'
    client: pg
    connection:
      host: 'localhost'
      port: 5432
      user: backstage
      password: secret
```