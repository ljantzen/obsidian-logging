# Olog - Log to Obsidian Journal from command line 

This little utility written in Rust makes it possible for me to log directly to todays daily note from the linux command line. 

## Environment variable

Olog expects the environment variable $OBSIDIAN_VAULT_DIR to be defined and point to the top level directory of the vault. 

## Configuration file

Olog reads ~/.config/olog/olog.toml on startup.  This is a file that uses the TOML configuration format.  See olog.example.toml for what can be configured.

My vault has a particular layout supported by Templater templates, and the example configuration reflects that.   

Olog looks for a marker that signifiies where the log entries block will start. Log entries must be consecutive without empty lines. The marker is specified in the config file.
In my case it is a level 2 heading with a clock emoji. 


## Command line switches 

### No swithces

When invoking the command `olog This is a log entry` olog will append the string `- HH:mm This is a log entry` (where HH:mm) is the current timestamp ) to the markdown daily note. 

### -t or --time 

The timestamp may be overridden by specifying the -t/--time HH:mm switch.  Log entries are sorted chronologically before being added to the md file. 


### -l  or --list 

You can list the current days log entries by specifying the -l option. 

### -b <days> or --back <days>

By specifying `-b <number>` you can go back in time and list the logs `number` of days ago. `olog -b 0` is the same as `olog -l`

### -u or --undo 

Removes the last log entry (undo)

### -e or --edit

Invokes $EDITOR with todays file.  Uses vim if $EDITOR is not set,
