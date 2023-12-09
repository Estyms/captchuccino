
# Captchuccino

A Self-Hosted Discord bot that adds a captcha to gatekeep a discord server

## Environment Variables

To run this project, you will need to add the following environment variables to your .env file

`DISCORD_TOKEN` : The token you'll use to run this bot.

`LANG` : The locale that the bot will use, either `fr` or `en` for now.

`GUILD_ID` : The ID of the server the bot will run in.

`ROLE_ID` : The ID of the Unverified role on your server.

`BOT_CHANNEL_ID` : The ID of the bot channel for logging messages

## Deployment

> ⚠️ The bot needs to have the SERVER MEMBERS INTENT enabled in the developper dashboard.

Get the docker image from [my registry](https://registry.regnault.dev)
```bash
  docker pull r.regnault.dev/captchuccino:latest
```

Launch the docker image with environment variables

```bash
  docker run r.regnault.dev/captchuccino:latest \
    -e DISCORD_TOKEN=<TOKEN> \
    -e LANG=<LANG> \
    -e GUILD_ID=<GUILD_ID> \
    -e ROLE_ID=<ROLE_ID> \
    -e BOT_CHANNEL_ID=<BOT_CHANNEL_ID>
```

