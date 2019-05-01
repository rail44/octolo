# Octolo

[![Build Status](https://travis-ci.org/rail44/octolo.svg?branch=master)](https://travis-ci.org/rail44/octolo) [![crates.io](https://img.shields.io/crates/v/octolo.svg)](https://crates.io/crates/octolo)

Octo To Local, Open files with local editor from GitHub web.

## Supported Platforms

### Operating Systems

* Linux
* Mac OS

### Editors

* Neovim via [neovim-remote](https://github.com/mhinz/neovim-remote)
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

# If you are using ghq, this property can be omitted
root = "/home/john/src"

# Keep configurations for editor your using

[[editors]]
kind = "visual-studio-code"
bin = "/usr/local/bin/code" # $(which code)

[[editors]]
kind = "neovim-remote"
bin = "/usr/local/bin/nvr" # $(which nvr) 

[[editors]]
kind = "jetbrains-ide"
name = "IntelliJ IDEA" # or "PHPStorm", "RubyMine" ... 
bin = "/usr/local/bin/idea" # $(which idea)
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
