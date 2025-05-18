# discord-tts-bot: Your Voice Channel Assistant

[![Test](https://github.com/tktcorporation/discord-tts-bot/actions/workflows/test.yml/badge.svg)](https://github.com/tktcorporation/discord-tts-bot/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/tktcorporation/discord-tts-bot/branch/master/graph/badge.svg?token=HB6NMTENNZ)](https://codecov.io/gh/tktcorporation/discord-tts-bot)

## Introduction
Welcome to `discord-tts-bot`! This innovative Discord bot uses text-to-speech (TTS) technology to bring your voice channels to life. Whether you're hosting a game night, a study group, or just hanging out, `discord-tts-bot` adds an extra layer of interaction to your Discord server.

## Key Features
- **High Precision Text-to-Speech:** Utilizes AWS Polly for high accuracy in reading both Kanji and English text, providing a natural voice experience across diverse language environments.
- **Text-to-Speech in Voice Channels:** Simply type, and the bot speaks in your voice channel.
- **Easy to Set Up:** A few steps and your bot is ready on your server.
- **Smart Voice Channel Interactions:** 
  - Welcomes users with "いらっしゃい" when they join
  - Says "いってらっしゃい" when users leave
  - Says "おやすみなさい" when users move to AFK channel
  - Says "おはようございます" when users return from AFK channel
  - All messages include the user's nickname or username for a personalized experience

## Getting Started

### Prerequisites
Before you begin, make sure you have:
- Docker and docker-compose installed.
- An AWS account.
- A Discord bot token.

### Quick Setup
To set up your `discord-tts-bot`, configure the following environment variables:

1. **`DISCORD_TOKEN`**: Obtain this token from Discord to authenticate and run your bot. It's essential for bot operation on your server.
2. **AWS Credentials** (for text-to-speech functionality using AWS Polly):
   - **`AWS_ACCESS_KEY_ID`**
   - **`AWS_SECRET_ACCESS_KEY`**
   - **`AWS_REGION`**: Set these to integrate AWS Polly for high-quality text-to-speech conversion.

### Running the Bot
1. Invite the bot to your Discord server.
2. Start the bot with `docker-compose run app /bin/bash -c "make run"`.
3. In Discord, type `/join` to have the bot join a voice channel.
4. Start typing in discord chat and hear your messages read aloud!

### Development and Contributions
- **Development:** Use `docker-compose run app /bin/bash` for a development environment.
- **Testing:** Run `make test` to execute tests.
- **Linting and Formatting:** Keep your code clean with `make watch`.
- **Package Installation:** Use `make install` to install development packages.

## Deployment
- Deploy on Heroku with easy steps detailed in the deployment section.
- Follow the release and deployment guide for smooth updates.

### Release & Deploy

1. `git switch master`
1. `git pull`
1. `git switch -c release/vx.x.x`
1. Bump up to vx.x.x in Cargo.toml
1. `git commit -m ':bookmark: vx.x.x'`
1. `git push --set-upstream origin release/vx.x.x`
1. Create Pull Request
1. Merge Pull Request

## Additional License Information

### Sound File Attribution
- **`sounds/shabeko_dayo.wav`**: This specific sound file was generated using CoeFontStudio.

## Try it Out!
Try out my [sample bot](https://discord.com/api/oauth2/authorize?client_id=798137406946934784&permissions=2184261184&scope=bot) and see `discord-tts-bot` in action!

## License
This project is proudly licensed under the MIT License. Check out the LICENSE file for more details.
