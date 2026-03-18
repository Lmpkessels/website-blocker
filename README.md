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

1. Install Rust (if not installed):  
   https://www.rust-lang.org/tools/install

2. Clone and build:

   ```bash
   git clone https://github.com/Lmpksels/blocker.git
   cd blocker
   cargo build --release
   ```

3. Copy the binary
   sudo cp -f target/release/blocker /usr/local/bin/blocker

## Usage

```bash
# Add a domain
sudo blocker add example.com
```

```bash
# Remove a domain
sudo blocker remove example.com
```

```bash
# List blocked domains
blocker list
```

```bash
# Temporary block
# Minutes
sudo blocker block 30 min
```

```bash
# Hours
sudo blocker block 2h
```

```bash
# Unblock (500 keystroke passphrase)
sudo blocker unblock
```

```bash
# Run as systemd service (recommended)
#1 access blocker.service
sudo nano /etc/systemd/system/blocker.service
```

```bash
#2 paste the following text into blocker.service
[Unit]
Description=Blocker Daemon
After=network.target

[Service]
ExecStart=/usr/local/bin/blocker daemon
Restart=always
User=root
WorkingDirectory=/root

[Install]
WantedBy=multi-user.target
```

```bash
#3 Reload system and enable service
sudo systemctl daemon-reexec
sudo systemctl daemon-reload
sudo systemctl enable blocker
```

```bash
#4 Start the service
sudo systemctl start blocker
sudo systemctl status blocker
```

## Future features

Making sure this runs after rebooting the system and killing the running process becomes harder for the given amount of time.

## Contribution

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## License

Licensed under [MIT License](./LICENSE-MIT). <br/>
© 2026 Luuk Kessels
