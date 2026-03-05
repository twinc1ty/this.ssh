# this.ssh

![this.ssh Banner](https://github.com/dh00mk3tu/this.ssh/blob/main/Images/this-banner.png)

![GitHub release (latest by date)](https://img.shields.io/github/v/release/dh00mk3tu/this.ssh)

### 🚀 Releases

<!--RELEASES-LIST-->

| Version | Date | Link |
|:-------------|:------------------|:------|
| release-v1.0.0 | 2025-08-20 | [View Release](https://github.com/dh00mk3tu/this.ssh/releases/tag/release-v1.0.0) |

<!--RELEASES-LIST-END-->

### Simple SSH Management Tool

`main` branch is used for the intro, setup guide and LICENCE information.

| Section                                                                           | Description                                       |
| --------------------------------------------------------------------------------- | ------------------------------------------------- |
| [Project Overview](#project-overview)                                             | Overview of the project and its purpose           |
| [Features - Roadmap](#features---roadmap)                                         | List of features and future plans                 |
| [Installation](#installation)                                                     | Instructions on how to install the project        |
| [Development Setup](https://github.com/dh00mk3tu/this.ssh/wiki/Development-Setup) | Instructions for building the project from source |

### Project Overview

It was a regular summer afternoon and I found myself in a predcament situation.
I was setting up my work repository and for that I had to setup a SSH key to be able to push my code to the remote work repository.
Now, though the process of setting up a SSH key is quite simple, and has been document so many times, it wasn't really an issue; I've done it before countless times but I still had to check the documentation just in case because I always forget the exact commands to use since it happens not that regularly.

This wasn't the actual problem though; I already had my personal SSH key setup and now I had another that was associated with work, both were loaded in my SSH agent and I had to switch between them all the time. After a while I was annoyed with it that I have to switch between the SSH keys, remember the exact commands to use, or visit some documentation/GPT to get the commands right.

I though to myself, "There must be a better way to manage SSH keys and connections", the asnwer was to use a config file but I didn't find it elegant enough for my needs. I wanted something that would allow me to easily add, remove, and list SSH hosts without having to manually edit a config file or remember the exact commands.

Hence, `this.ssh` was born.

`this.ssh` is a simple SSH management tool that allows users to manage their SSH connections and configurations easily. It provides a user-friendly interface for adding, removing, and listing SSH hosts.

### Features - Roadmap

| Features - Complete                | Release Date |
| ---------------------------------- | ------------ |
| List SSH Keys                      | 30/07/2025   |
| Onload SSH Keys into the SSH agent | 30/07/2025   |
| Create SSH Keys                    | 13/08/2025   |
| Remove SSH Keys                    | 19/08/2025   |
| Documentation - Initial Phase      | 19/08/2025   |

---

| Features - Inprogress               | Release Date |
| ----------------------------------- | ------------ |
| Offload SSH Keys into the SSH agent | TBR          |
| Create User/User Auth               | TBR          |
| Save your keys to the cloud         | TBR          |

### Installation

Installation guide Will be updated soon once the first release is out.
