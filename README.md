# discord-speech-bot

## Prerequirements

- Docker, docker-compose
- AWS Account
- Discord Bot Token

## Get Started

### Env Vars

1. `cp -p .envrc.sample .envrc` and set variables.
1. Install [direnv](https://github.com/direnv/direnv).
1. `direnv allow`

### Run

1. Invite your bot to your server.
1. `docker-compose run app`
1. Post `~join` in your server.
1. The bot talk in voice chat.

### Develop

1. `docker-compose run app /bin/bash`

## Deploying to Heroku

### Prerequirements

- Heroku Account

### Deploy

```bash
docker-compose run heroku /bin/bash
```

```bash:in_container
heroku create <APP_NAME>
heroku login
apk add docker
heroku container:login
heroku container:push app -a <APP_NAME>
heroku container:release app -a <APP_NAME>
heroku ps:scale app=1 -a <APP_NAME>
```

## To invite sample bot
https://discord.com/api/oauth2/authorize?client_id=798137406946934784&permissions=2184261184&scope=bot

## LICENCE
MIT

### file

binaries/shabeko_dayo.wav is generayted by CoeFontStudio.