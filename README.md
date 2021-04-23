# Laulud

[laulud.lucaspickering.me](https://laulud.lucaspickering.me/)

A simple app for tagging songs/albums/etc. in Spotify. Log in with your Spotify account, no account setup needed.

## Development

You can run this with docker-compose

```sh
docker-compose up
```

Then it will be accessible at `https://localhost:3000`. Note the http**s**!

### Relay Compilation

The UI uses [Relay](https://relay.dev), which requires its own compiler step to compile GraphQL queries into TypeScript type definitions. When running the docker-compose stack, the UI container will automatically recompile your queries on changes. If you want to recompile without running the docker-compose stack though, you can run `npm run relay:watch` in a terminal window.

## Deployment

Deployed through [Keskne](https://github.com/LucasPickering/keskne). Images are automatically built and pushed on every push to master.
