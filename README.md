# openvpn manager

## Installation

- `cargo install openvpn_manager`

## Usage

### openvpn3

- `openvpn_manager 3 start file.ovpn`
- `openvpn_manager 3 stop file.ovpn`

### openvpn (version 2)

- `openvpn_manager 2 start file.ovpn` # only start in this version

## TODO

- [ ] Add tests
- [ ] Background process for openvpn (version 2)
    - [ ] Implement stop command
- [ ] Publish
    - [ ] Publish Crate to Crates.io
    - [ ] Publish in AUR (Arch)
- [ ] Implement command `myip` to show actual ip public
- [ ] Implement command `status` to show actual status (connect to vpn or not)
- [ ] Implement command `ping` to show ping (this is a wrapper command to `ping google.com`), maybe

## Contribution

Welcome

### Development

#### Commands

- `sudo cargo run -- 2 start file.ovpn` # only start in this version
