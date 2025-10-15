# OpenVPN Manager

## Installation

- cargo: `cargo install openvpn-manager`
  - crates.io page: https://crates.io/crates/openvpn-manager
- Arch Linux (AUR): like `paru -S openvpn-manager-git`
  - AUR page: https://aur.archlinux.org/packages/openvpn-manager-git

## Usage

### openvpn3

- `openvpn-manager start 3 file.ovpn`
- `openvpn-manager stop 3 file.ovpn`
- `openvpn-manager status 3 file.ovpn`

### openvpn (version 2)

- `openvpn-manager start 2 file.ovpn` # only start/status in this version
- `openvpn-manager start 2 file.ovpn --auth-file credentials.txt` # with username/password authentication
- `openvpn-manager status 2`

For services like NordVPN that require authentication, create a credentials file with your username on the first line and password on the second line, then use the `--auth-file` option.

## TODO

- [ ] Background process for openvpn (for version 2)
    - [ ] Implement stop command for version 2
- [x] Publish
    - [x] Publish Crate to Crates.io
    - [x] Publish in AUR (Arch)
- [ ] Implement command `myip` to show actual ip public
- [x] Implement command `status` to show actual status (connect to vpn or not)
- [ ] Implement command `ping` to show ping (this is a wrapper command to `ping google.com`), maybe

## Contribution

Everyone is welcome.

### Development

#### Commands

- `sudo cargo run -- start 2 file.ovpn` # only start in this version
- `sudo cargo run -- start 2 file.ovpn --auth-file credentials.txt` # with authentication
