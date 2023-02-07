<div align="center">

![](./.github/nodem.png)

# **NODEM**

</div>

## **Table of contents**

- [Intro](#intro)
- [Usage](#usage)
- [Iteration Plan](#iteration-plan)
- [Licenses](#licenses)

## **Intro**

🦀 Tool to practice Rust, which manages different versions of Node, cross-platform

## **Usage**

| Command     | Description                                                                   | Example                          |
| ----------- | ----------------------------------------------------------------------------- | -------------------------------- |
| `info`      | Display information of the machine on which Node is running                   | `nodem info`                     |
| `list`      | List the installations. Type "remote" at the end to see what can be installed | `nodem list` `nodem list remote` |
| `install`   | Specify the version of Node you want to install.                              | `nodem install 18.14.0`          |
| `uninstall` | Uninstalls the specified version of Node.js                                   | `nodem uninstall 18.14.0`        |
| `use`       | Switch to use the specified version. Optionally use "latest".                 | `nodem use`                      |
| `help`      | Shows help about the selected command.                                        | `nodem help`                     |
| `version`   | Display the version of the tool.                                              | `nodem version`                  |

## **Iteration Plan**

- [ ] INSTALLER
  - [ ] 🏃 Windows
  - [ ] Linux
  - [ ] MacOs
- [ ] COMMANDS
  - [x] info
  - [ ] 🏃 list
  - [ ] install
  - [ ] uninstall
  - [ ] use

## Licenses

This is an Open Source project under MIT licenseThis is an Open Source project under MIT license [here](./LICENSE)
