# Octolo

[![Build Status](https://travis-ci.org/rail44/octolo.svg?branch=master)](https://travis-ci.org/rail44/octolo) [![crates.io](https://img.shields.io/crates/v/octolo.svg)](https://crates.io/crates/octolo)

Octo To Local, Open files with local editor from GitHub web.

## Supported Platforms

### Operating Systems

* Linux
* Mac OS

### Editors

* Neovim
* Visual Studio Code
* JetBrains IDEs

### Browsers

* Firefox
* Chrome
* Chromium

> note: Although not well tested, Opera, Edge and Vivaldi may run with configuration that same as Chrome.

## Installation

### Install Native Application

Because we use Native Messaging API, you should install native application.

#### with curl

Executable binary is available for each releases.

###### for Linux

```sh
# In any directory conteined in $PATH
$ curl -Lo octolo https://github.com/rail44/octolo/releases/latest/download/octolo-x86_64-unknown-linux-musl
$ chmod a+x octolo
```

###### for Mac OS

```sh
# In any directory conteined in $PATH
$ curl -Lo octolo https://github.com/rail44/octolo/releases/latest/download/octolo-x86_64-apple-darwin
$ chmod a+x
```

#### with Cargo

```sh
$ cargo install octolo
```

### Configuration

Create `~/.config/octolo/octolo.toml` like bellow.

```toml
# Comment out browsers your using
browser_list = [
    # "Firefox",
    # "Chromium",
    # "Chrome"
]

# If you have configuration ghq.root, Octolo will use it as default
# root = "/home/john/src"

# path = github.com/{{user}}/{{repository}}

# Use configurations for editor your using
[[editors]]
kind = "neovim"
# The address that nvim listening. By default, Octolo uses $NVIM_LISTEN_ADDRESS 
# address = "" 

[[editors]]
kind = "visual-studio-code"
bin = "/usr/local/bin/code" # Path of $(which code)

[[editors]]
kind = "jetbrains-ide"
name = "IntelliJ IDEA" # or "PHPStorm", "RubyMine" ... 
bin = "/usr/local/bin/idea" # Path of $(which idea)
```

**Once creating configuration, run bellow**

```sh
$ octolo cofig dump
# Dump full config with optional fields...

$ octolo manifest
# Dump Native Messaging Manifests for each browsers

$ octolo manifest -w
# And place it!
```

Manifests contains absolute path for octolo.
When you change location of it, you should redo above.

### Install WebExtension

Firefox: [https://addons.mozilla.org/ja/firefox/addon/octolo/](https://addons.mozilla.org/ja/firefox/addon/octolo/)  
Chrome: [https://chrome.google.com/webstore/detail/octolo/igdmgdknajejkdpaonpnpjedakhppiob](https://chrome.google.com/webstore/detail/octolo/igdmgdknajejkdpaonpnpjedakhppiob)

Then, you can open files by right-click menu from GitHub Web!

## Bulid

### WebExtension

```sh
# In `extension` dir
$ npm install
$ npm run build
```

### Native 

```sh
# In `native` dir
$ cargo build
```
