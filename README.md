# JustFetch
A simple minimal neofetch-like alternative, aimed at one thing: Just fetching information about your system, while being easy to configure.

Nothing more, nothing less.

## Features
- Fetching common system properties, such as your username;
- A very simple config;
- Color support;
- Support for specifying a custom shell-command to run and print out

## Constant Aliases
- `[host]` - System host name
- `[kernel]` - Currently active kernel
- `[username]` - Your username
- `[shell]` - Currently active shell
- `[distro]` - Active distribution name
- `[distro_id]` - Distribution ID, for example `arch`
- `[distro_build_id]` - Distribution Build ID, for example `rolling`
- `[total_mem]` - Total amount of installed memory
- `[cached_mem]` - Cached amount of memory
- `[available_mem]` - Available memory
- `[used_mem]` - Used memory

## Custom Commands
> **Warning**:
> Custom Commands add a lot of overhead, use them sparingly.

You may display text from external shell-commands by using `$cmd=[command]`.

For example: `$cmd=uname -o` - Will display the operating system you are using.

If you don't use custom commands and want to exclude them entirely, add the `--no-cmd` argument when launching JustFetch.

## Custom Color
You may use colors in your config by defining the text and then the RGB, like such: `rgb["Hello, I'm red!", 255, 0, 0]`, which applies a red color to the text inside.
