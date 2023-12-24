# Juno

## Introduction

This is a CLI music player, It can play local music or remote stream and it can
output the sound to the local sound server or as an HTTP stream. This is a
standalone project but it also has a [Web Client](https://codeberg.org/aleidk/fuuka) with further features.

## Features

- Play local files or remote streams (like youtube streams).
- Local or remote (HTTP) audio output.
- [MPRIS](https://wiki.archlinux.org/title/MPRIS) support
- Dynamic server-client design:
    - On start it attach to a socket, a local unix file socket (default) or a port to allow for remote connections ([MPD](https://wiki.archlinux.org/title/Music_Player_Daemon) like).
    - If a second instance is invoque on the same socket, it will act as a client to the process running on that socket, allowing to modify the playback and exiting immediately ([MPC](https://www.musicpd.org/clients/mpc/) like).
    - By default the server process will end when the queue is empty unless it's ran in _"daemon"_ mode. This allows to continue using the clients to add more music later.

## FAQ

### Why Rust?

A core requirement is to handle and administrate masive music queues, so the
project needs an efficient language, I also wanted a compiled language for
easier binary distribution and dependency management. By the quote I read
somewhere of _"un-optimized Rust code is faster than optimized Go code"_, I
decided to use Rust.

### Why the name?

#### TLDR

The name comes from [Juno](https://megamitensei.fandom.com/wiki/Juno), the Persona of Fuuka Yamagishi in Persona 3.
Fuka is also a [complementary project](https://codeberg.org/aleidk/fuuka) for this music player.

#### Long Version

One of my first projects was a discord bot that acted as a frontend of
[MPD](https://wiki.archlinux.org/title/Music_Player_Daemon) while also played the HTTP stream in the voice call.

I called the bot fuuka because you actually have to talk to the bot to ask for
music like you do in the game. This is the 3rd iteration of the idea (and
hopefully the definitive), so I decided to maintain the name in it's honor.

The project was split into 2 though:
- [Fuuka](https://megamitensei.fandom.com/wiki/Fuuka_Yamagishi), the navi of SEES in Persona 3, you can ask her to change the musing in tartarus. It act as the frontend to interact with the player.
- [Juno](https://megamitensei.fandom.com/wiki/Juno), the persona of Fuuka that grants her the ability to communicate telepatically to her teamates. It act as the music player.

## Similar projects

- [Navidrome](https://www.navidrome.org)
- [Azuracast](https://www.azuracast.com/)
- [Cadence](https://github.com/kenellorando/cadence) (Use Icecast and Liquidsoap)
- [forte](https://github.com/kaangiray26/forte)
- [Simple MPD in Rust](https://dev.to/tsirysndr/how-i-built-a-simple-music-player-daemon-in-rust-with-a-cliweb-ui-51e0)

