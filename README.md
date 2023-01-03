# JustFetch
A simple minimal neofetch-like alternative, aimed at one thing: Just fetching information about your system.

Nothing more, nothing less.

## Features
- Fetching common system properties, such as your username;
- Simple config;
- Support for specifying a custom shell-command to run and print out

## Constant Aliases
`[host]` - System host name
`[kernel]` - Currently active kernel
`[username]` - Your username
`[shell]` - Currently active shell

## Custom Commands
You may display text from external shell-commands by using `$cmd=[command]`.

For example: `$cmd=uname -o` - Will display the operating system you are using.
