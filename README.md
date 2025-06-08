# Olog - Log to Obsidian Journal from command line 

[![Crates.io](https://img.shields.io/crates/v/olog.svg)](https://crates.io/crates/olog)
[![Documentation](https://docs.rs/olog/badge.svg)](https://docs.rs/olog)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This little utility written in Rust makes it possible for me to log directly to todays daily note from the linux command line. 

## Installation

You can install olog using cargo:

```bash
cargo install olog
```

Or build from source:

```bash
git clone https://github.com/ljantzen/olog
cd olog
cargo build --release
```

The binary will be available at `target/release/olog`.

## Environment variable

Olog expects the environment variable $OBSIDIAN_VAULT_DIR to be defined and point to the top level directory of the vault. 

## Configuration file

Olog reads ~/.config/olog/olog.yaml on startup.  This is a file that uses the yaml configuration format.  See olog.example.yaml for what can be configured.

My vault has a particular layout supported by Templater templates, and the example configuration reflects that.   

Olog looks for a marker that signifiies where the log entries block will start. Log entries must be consecutive without empty lines. The marker is specified in the config file.
In my case it is a level 2 heading with a clock emoji. 


## Command line switches 

### No swithces

When invoking the command `olog This is a log entry` olog will append the string `- HH:mm This is a log entry` (where HH:mm) is the current timestamp ) to the markdown daily note. 

### -t or --time 

The timestamp may be overridden by specifying the -t/--time HH:mm switch.  Log entries are sorted chronologically before being added to the md file. 


### -l  or --list 

You can list the current days log entries by specifying the -l option.  If olog is invoked without any arguments, this is the default action.

### -b <days> or --back <days>

By specifying `-b <number>` you can go back in time and list the logs `number` of days ago. `olog -b 0` is the same as `olog -l`

### -u or --undo 

Removes the last log entry (undo)

### -e or --edit

Invokes $EDITOR with todays file.  Uses vim if $EDITOR is not set,

### -h or --help 

Print command line argument help

### -T list mode 

Specifies the list output mode when olog -l is called. Valid arguments are -T bullet and -T table. Overrides the list mode in olog.yaml configuration file

### -v or --version 

Outputs the current version string and exits execution



## Screenshots

### Bullet mode 

![image](https://github.com/user-attachments/assets/72c50c59-5185-4cb4-a871-473a8fd8b96f)

### Table mode 

![image](https://github.com/user-attachments/assets/ad3fe2c4-9a33-4272-a059-3d22617cef97)


# Contact info 

https://mas.to/@jantzten
https://bsky.app/profile/leif.jantzen.no
