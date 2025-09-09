# Obsidian logging - Log to Obsidian Journal from command line 

[![Crates.io](https://img.shields.io/crates/v/obsidian-logging.svg)](https://crates.io/crates/obsidian-logging)
[![Documentation](https://docs.rs/obsidian-logging/badge.svg)](https://docs.rs/obsidian-logging)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This little utility written in Rust makes it possible to log directly to todays daily note from the linux command line. 

## Installation

You can install obsidian-logging using cargo:

```bash
cargo install obsidian-logging
```

Or build from source:

```bash
git clone https://github.com/ljantzen/obsidian-logging
cd obsidian-logging
cargo build --release
```

Or download binaries directly from the [github releases page](https://github.com/ljantzen/obsidian-logging/releases).


## Tip

Since obsidian-logging is quite a mouthful to type every time, it is recommended to create a short alias.  E.g

```
alias q=` obsidian-logging`
```
(Note: the space before the program name stops the commaand from being entered into command history)

On Windows, the `doskey` command can be used to create a macro: 

```
doskey q=obsidian-logging
```

## Configuration file

Obsidian-logging reads ~/.config/obsidian-logging/obsidian-logging.yaml on startup.  If it does not exist, obsidian-logging will create it and prompt for some of the values. 
This is a file that uses the yaml configuration format.  See obsidian-logging.example.yaml for what can be configured. 

Obsidian-logging looks for a marker that signifiies where the log entries block will start. Log entries must be consecutive without empty lines. The marker is specified in the config file. 

## Environment variable 

If specified, $OBSIDIAN_VAULT_DIR will override the `vault` value in `obsidian-logging.yaml`


## Command line switches 

### No arguments 

When invoking `obsidian-logging` without any arguments the entries of the current day, if any, will be listed.  This is equivalent to `obsidian-logging -l`

### No swithces

When invoking the command `obsidian-logging This is a log entry` obsidian-logging will append the string `This is a log entry` to the log section of the markdown daily note. 
A timestamp will be prepended according to the chosen list mode. If list mode is `bullet`, '- HH:mm ' is prepended to the log statement.  If list mode is 'table', the log statement is 
wrapped in markdown table column separators:  `| HH:mm | log statement|`

### -t or --time 

The timestamp may be overridden by specifying the -t/--time HH:mm switch.  Log entries are sorted chronologically before being added to the markdown file. 


### -l  or --list 

You can list the current days log entries by specifying the -l option.  If obsidian-logging is invoked without any arguments, this is the default action.

### -b <days> or --back <days>

By specifying `-b <number>` you can go back in time and list the logs `number` of days ago. `obsidian-logging -b 0` is the same as `obsidian-logging -l`

### -e or --edit

Invokes $EDITOR with todays file.  Uses vim if $EDITOR is not set,

### -f or --time-format 

Specifies 12H or 24H time format.  24H is default.   Overrides `time_format` in obsidian-logging.yaml 

### -s or --silent 

Do not output anything, not even error messages 

### -h or --help 

Print command line argument help

### -T list mode 

Specifies the list output mode when obsidian-logging -l is called. Valid arguments are -T bullet and -T table. Overrides the list mode in obsidian-logging.yaml configuration file

### -v or --version 

Outputs the current version string and exits execution

### -S or --stdin 
If specified, input will be read from stdin instead of command line arguments, allowing piping of log statements into the program.  Carriage return or linefeed characters will be removed, and log statement will be logged as a single line. 


## Screenshots

### Bullet mode 

![image](https://github.com/user-attachments/assets/72c50c59-5185-4cb4-a871-473a8fd8b96f)

### Table mode 

![image](https://github.com/user-attachments/assets/ad3fe2c4-9a33-4272-a059-3d22617cef97)


# Contact info 

https://mas.to/@jantzten

https://bsky.app/profile/leif.jantzen.no
