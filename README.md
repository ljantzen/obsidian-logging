# Olog - Log to Obsidian Journal from command line 

This little utility written in Rust makes it possible for me to log directly to todays daily note from the linux command line. 

My vault has a particular layout supported by Templater templates.   The Dirary template automatically creates the required directory-structure, file, and markdown content. 

The directory and filename is calculated based on the date: 

$OBSIDIAN_VAULT/10-Journal/YYYY/MM/YYYY-MM-DD.md

The last line of a fresh diary markdown file is a level 2 header: 

```
## 🕗
```

This header signifies the start of the log of the day.  


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
