# Laulud

[laulud.lucaspickering.me](https://laulud.lucaspickering.me/)

A simple app for tagging songs/albums/etc. in Spotify. Log in with your Spotify account, no account setup needed.

## Development

### Create a Spotify App

Before you can run this locally, you'll need to create a Spotify developer app that you can use to auth with the Spotify API. Go to the [Spotify Developer Dashboard](https://developer.spotify.com/dashboard/applications) and create a new app with any name and description. For the redirect URI, put:

```
https://localhost:3000/api/oauth/callback
```

Then, note the client ID and secret from the new app page.

Create a file in the repo root named `.env`. Add these contents:

```
ROCKET_SPOTIFY_CLIENT_ID=<copy from Spotify>
ROCKET_SPOTIFY_CLIENT_SECRET=<copy from Spotify>
```

Make sure to do this _before_ running the Docker stack. If you change the `.env` file while the stack is running, you'll need to restart the whole stack to get Docker to load the new environment variables and pass them to the API container.

### Running

You can run this with docker-compose

```sh
docker-compose up
```

Then it will be accessible at `https://localhost:3000`. Note the http**s**!

### Relay Compilation

The UI uses [Relay](https://relay.dev), which requires its own compiler step to compile GraphQL queries into TypeScript type definitions. When running the docker-compose stack, the UI container will automatically recompile your queries on changes. If you want to recompile without running the docker-compose stack though, you can run `npm run relay:watch` in a terminal window.

## Deployment

Deployed through [Keskne](https://github.com/LucasPickering/keskne). Images are automatically built and pushed on every push to master.
