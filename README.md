# JustFetch
A simple minimal neofetch-like alternative, aimed at one thing: Just fetching information about your system, while being easy to configure and being blazingly fast.

Nothing more, nothing less.

## Features
- Fetching common system properties, such as your username
- A **very** simple config
- Easy color support
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
You may display text from shell commands with ease, and it's supported by default as long as you don't pass `--raw`.

By default, the config is a mix of bash and a custom format. So you can add `$(whoami)` to it and it'll return your username.

If no commands are detected, JustFetch automatically acts like as if you passed `--raw` to skip any additional overhead.

## Custom Color
You may use colors in your config by defining the text and then the RGB, like such: `rgb["Hello, I'm red!", 255, 0, 0]`, which applies a red color to the text inside.

There is no limit to how many colors you may use on one line. Doing something like:

`rgb["Hello, I'm red!", 255, 0, 0] rgb["Hello, I'm green!", 0, 255, 0]` will work just fine.

## Focus
JustFetch focuses **a lot** on performance, stability and simplicity, and it does this by:

1. Not using external processes to capture the output of shell commands by default, as that's slow
   - If you do use shell commands via `$(...command...)`, it will add a bit of overhead, but far less than other fetching programs.
2. Using a custom crate for fetching system information in a straight-forward and fast way
3. Making use of a super simple config, requiring no libraries outside of regular expressions for color lookups.
4. Endless manual performance benchmarks, both against other compiled versions of itself, but also against other fetching programs.
5. Keeping the codebase incredibly small and easy-to-read, which in turn makes it a lot easier to optimize and maintain.
6. Not introducing breaking changes, not bloating itself and not making things more complicated than they have to be.
7. Making **very limited** usage of regular expressions, as they aren't the fastest and not the easiest to read.
   - Currently it's only being used for the color lookups.
