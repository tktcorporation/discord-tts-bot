# discord-speech-bot

[![Test](https://github.com/tktcorporation/discord-speech-bot/actions/workflows/test.yml/badge.svg)](https://github.com/tktcorporation/discord-speech-bot/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/tktcorporation/discord-speech-bot/branch/master/graph/badge.svg?token=HB6NMTENNZ)](https://codecov.io/gh/tktcorporation/discord-speech-bot)

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

<!-- 1. `docker-compose run app bash -c "RUST_BACKTRACE=1 cargo run"` docker for windows only? -->

#### Test

```bash
RUST_BACKTRACE=1 cargo test --all-features
```

#### Linter, Formatter

- Lint

```bash
cargo clippy --all
```

- Format

```bash
cargo fmt --all
```

##### Task Runner

[act](https://github.com/nektos/act) can use as a task runner in this project.  
But, it takes longer than `cargo` commands.

```bash
# lint, format(check), test
act
```

```bash
# deploy to heroku
act release
```

## Deploying to Heroku

### Prerequirements

- Heroku Account

### Release & Deploy

1. `git switch master`
1. `git pull`
1. `git switch -c release/vx.x.x`
1. Bump up to vx.x.x in Cargo.toml
1. `git commit -m 'vx.x.x'`
1. `git push --set-upstream origin release/vx.x.x`
1. Create Pull Request
1. Merge Pull Request

## To invite sample bot
https://discord.com/api/oauth2/authorize?client_id=798137406946934784&permissions=2184261184&scope=bot

## LICENCE
MIT

### file

sounds/shabeko_dayo.wav is generayted by CoeFontStudio.