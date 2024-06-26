# lyrebird

An innocent discord bot that will never call `youtube-dl`

This application is dual licensed under the Apache license, version 2.0 or the MIT license, at your option.

# Features

 * Play anything that `yt-dlp` supports
 * Because it is self-hosted, the bot is yours, you can run it to keep it in voice 24/7 and there are no
limits in number of songs in queue.
 * Uses symphonia for decoding, does not need ffmpeg to work.
 * Search on YouTube and then select songs you want to enqueue

# Build instructions

First, ensure that you have the latest Rust stable installed. On distributions such as Debian and Ubuntu,
as well as on Windows, you can use [rustup] to manage and install your Rust toolchains.


## Setup the environment on Linux

Make sure you have `gcc`, `cmake`, `libopus`, and `shuttle` installed. Ensure that you are using the latest
version for `yt-dlp`, otherwise downloading YouTube audio might not work correctly.

## Setting up the environment on Windows

Running the bot on windows is not tested.

## Running the bot

Create a `secrets.toml` file according to the `secrets.toml.example`. Afterwards, run the bot using the
`shuttle` `service` See [this tutorial][adding bot to servers] on how to add your bot to your discord
server. Make sure that "Message Content Intent" is set as enabled in the developer portal so the bot can
read messages.

You can use the following URL for adding your bot to your server:

```
https://discord.com/oauth2/authorize?client_id=XXXXXXX&permissions=549792514304&scope=bot
```

After adding your bot to your server, you need to register the slash commands that the bot supports. Using
the account that matches the `owner_id` set in your secrets.toml file, you can run the `~register` command to either
register it in the guild specifically or globally.

# Commands

* `~register`, for use by bot owners only. For first run, use this to register slash commands on your server.
When updating the bot, use `~register` to reregister for updating the slash commands.
* `/join` - tell the bot to join your current voice channel.
* `/leave` - leaves the current vc.
* `/playurl <url>` - add a URL to the queue. Anything that yt-dlp supports are supported.
* `/play <term>` - search on YouTube and add the first search result to the queue.
* `/playrange <url> <range>` For playlists, specify which songs to play. This corresponds to the `-I RANGE`
command line argument for `yt-dlp`. Values are comma-separated, ranges use `:`. Example: `1,3,5:6`. Ranges
without a lower or upper bound are also supported. (`:3` means up to the third song, and `3:` means all starting
from the third song)
* `/playall <url>` Enqueues all songs from a playlist specified at the URL
* `/playrand <url> <num>` Fetches all songs in the playlist, but take a random amount of songs from the list.
* `/search <term> [num]` Searches a given term on YouTube and returns the first `num` results. `num` defaults to
5 and cannot be greater than 25. Will include a selection menu for which songs in the result you'd like to enqueue.
* `/queue [page]` Lists current songs queued. 10 songs are displayed per page. You can specify the page in the
optional argument. By default displays the first page.
* `/shuffle` - Shuffles the queue.
* `/clear` - Stops the current song and clear all songs in the queue.
* `/rm <index>` - Removes a specific song in the queue.
* `/skip` - skips the current song and play the next one in queue.
* `/move <from> <to>` moves a song at the index to a new index.
* `/swap <a> <b>` swaps two songs' positions in the queue.
* `/pause` pauses the current playback
* `/resume` resumes the current song
* `/deafen` and `/undeafen` - historial artifact. Planned for removal


[adding bot to servers]: https://discordjs.guide/preparations/adding-your-bot-to-servers.html
[rustup]: https://rustup.rs/
[shuttle]: https://shuttle.rs/