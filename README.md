# Olog - Log to Obsidian Journal from command line 

This little utility written in Rust makes it possible for me to log directly to todays journal markdown file from the linux command line. 

My vault has a particular layout supported by Templater templates.   The Dirary template automatically creates the required directory-structure, file, and markdown content. 

The directory and filename is calculated based on the date: 

$OBSIDIAN_VAULT/10-Journal/YYYY/mm/YYYY-MM-dd.md

The last line of a fresh diary markdown file is a level 2 header: 

```
## 🕗
```

This header signifies the start of the log of the day.  

When invoking the command `olog This is a log entry` olog will append the string `- HH:mm This is a log entry` (where HH:mm) is the current timestamp ) to the markdown diary file. 

The timestamp may be overridden by specifying the -t/--time HH:MM switch.  Log entries are sorted chronologically before being added to the md file. 

You can list the current days log entries by specifying the -l option.  By specifying `-n <number>` you can list the logs `number` of days ago.

`olog -n 0` is the same as `olog -l`
