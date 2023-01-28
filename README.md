# JustFetch
A simple minimal neofetch-like alternative, aimed at one thing: Just fetching information about your system, while being easy to configure.

Nothing more, nothing less.

## Features
- Fetching common system properties, such as your username;
- Simple config;
- Support for specifying a custom shell-command to run and print out

Oh and it's fast and small, probably faster than most other fetching programs out there.

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
You can specify a custom output color for the whole message by passing `--red` / `--green` / `--blue` with a value from `0` to `255`, default is `255` for all 3.

For example: `./just-fetch --red 0 --green 255 --blue 0`
