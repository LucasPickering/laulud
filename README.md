# Laulud

[laulud.lucaspickering.me](https://laulud.lucaspickering.me/)

A simple app for tagging songs/albums/etc. in Spotify. Log in with your Spotify account, no account setup needed.

## Development

You can run this with docker-compose

```sh
docker-compose up
```

Then it will be accessible at `https://localhost:3000`. Note the http**s**!

## Deployment

Deployed through [Keskne](https://github.com/LucasPickering/keskne). To push changes:

```sh
docker-compose -f docker-compose.build.yml build
docker-compose -f docker-compose.build.yml push
```

Then redeploy via Keskne.
