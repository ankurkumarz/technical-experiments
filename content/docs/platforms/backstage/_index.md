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
      password: ****
```

## Configuring Sonar
- [Install Sonar Frontend Plugin](https://github.com/backstage/backstage/tree/master/plugins/sonarqube)
- [Install Sonar Backend Plugin](https://github.com/backstage/backstage/tree/master/plugins/sonarqube-backend)
- Generate User Token from Sonar (Login Required): My Account -> Security (snapshot below)
  - Sonar API works without any password if you have the user token
![Sonar User Token](sonar-token.png)
- Ensure that plugin got installed correctly by running following - it should be in :
```
yarn tsc
find . -name '*sonar*'
```
- You should see following files:
```
./node_modules/@backstage/plugin-sonarqube
./node_modules/@backstage/plugin-sonarqube-backend
./packages/backend/src/plugins/sonarqube.ts
```
- Get the Sonar Project Key from **Dashboard** -> *Project* -> Project Information (see below)
![Sonar Project Key](sonar-project-key.png)
- Update sonar settings in *app-config.yaml*:
```
sonarqube:
  instances:
    - name: default
      baseUrl: http://localhost:9000
      # API Key is the user token generated from Sonar
      apiKey: ************
```
- Put the configuration in your project *catalog-info.yaml*:
```
  annotations:
    backstage.io/techdocs-ref: dir:.
    jenkins.io/job-full-name: "devlocal:cloudcost-inspector"
    sonarqube.org/project-key: "com.ps.cloudcostinspector:CloudCostInspector"
```

## Configuring Jenkins