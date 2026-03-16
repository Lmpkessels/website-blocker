# Blocker

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Built with Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Status: In Progress](https://img.shields.io/badge/Status-In%20Progress-orange)](https://github.com/Lmpksels/blocker)

**Blocker** is a simple, lightweight CLI website blocker written in Rust for Linux.  
It modifies `/etc/hosts` to block distracting domains for a chosen amount of time, helping you stay focused.

## Features

- Add/remove domains from `/etc/hosts`
- List currently blocked domains
- Temporarily block all added domains for a set duration (e.g. 30min, 2h)
- Protected unblocking with a passphrase (prevents impulse decisions during focus sessions)

## Installation

1. Make sure you have Rust installed:  
   https://www.rust-lang.org/tools/install

2. Clone and build:

   ```bash
   git clone https://github.com/Lmpksels/blocker.git
   cd blocker
   cargo build --release
   ```

## Usage

```bash
# For sudo access
sudo cp -f target/release/blocker /usr/local/bin/blocker

# Add a domain (blocks it permanently until removed)
# Automatically adds: 127.0.0.1 example.com and 127.0.0.1 www.example.com
sudo blocker add example.com

# Remove a domain
sudo blocker remove example.com

# List all currently blocked domains
# (no sudo needed for read-only commands)
blocker list

# Temporarily block all added domains
# Supported units: min, h(minutes, hours)
sudo blocker block 30min
sudo blocker block 2h


# Unblock (requires passphrase)
# You need to enter a 500 word passphrase passphrase
sudo blocker unblock
```

When you stop the process through CTRL-C you should manually use sudo nano /etc/hosts to remove the blocked domains, because the program was stoped earlier.

## Future features

Making sure this runs after rebooting the system and killing the running process becomes harder for the given amount of time.

## Contribution

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## License

Licensed under [MIT License](./LICENSE-MIT). <br/>
© 2026 Luuk Kessels
