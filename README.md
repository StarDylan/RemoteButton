# Remote Button

[![License](https://img.shields.io/badge/license-GPLv3-blue.svg)](LICENSE)

## Overview

The "Remote Button" project allows button presses to be sent across computers using an MQTT relay. This project is designed to provide a simple and efficient way to remotely trigger actions or events on different devices. 

## Features

- MQTT protocol for communication
- Customizable button mappings


## Getting Started

### Pre-Requisites
This currently in only built for Windows and macOS, for other systems you can install from source.

### Installing + Setup

1. Download the proper binary for your system (Windows or macOS) from the releases tab.

2. Install in the designated place for your system.

3. Run `rb` to generate default config file

4. Modify the `mqtt_server_host` and `mqtt_server_port` to point to an MQTT server, here you can also modify the key bindings.

5. Run `rb send` on one computer

## Config File

`mqtt_server_host` `mqtt_server_port` - Standard connection details for the MQTT server.

`send_map` - The keys which are listened to and what keys to send (any keys not listed here won't be sent to the receiver)

`recv_map` - The keys received from the sender to follow and which key to map to.

-   Example: `KeyA = "KeyB"` will map any received A's into B's.
- See [here]([https://docs.rs/rdev/latest/rdev/enum.Key.html) for a list of valid keys.

## Contributing
Contributions are welcome! Please create an issue and submit a PR with changes.