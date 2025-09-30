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
(Note: the space before the program name stops the command from being entered into command history)

On Windows, the `doskey` command can be used to create a macro: 

```
doskey q=obsidian-logging
```

## Configuration file

Obsidian-logging reads ~/.config/obsidian-logging/obsidian-logging.yaml on startup.  If it does not exist, obsidian-logging will create it and prompt for some of the values. 
This is a file that uses the yaml configuration format.  See obsidian-logging.example.yaml for what can be configured. 

Obsidian-logging looks for a marker that signifies where the log entries block will start. Log entries must be consecutive without empty lines. The marker is specified in the config file.

### Category-specific section headers

The default block marker is specified in the configuration file with the property `section_header`. 

Example configuration:

```yaml
section_header: "## 🕗"
```

obsidian-logging can log to a number of different sections in the daily log. These are called log `categories`.  You can define category-specific section headers in the config yaml using the pattern `section_header_<category_name>`. When using the `-c` or `--category` flag, obsidian-logging will look for a section header with this pattern and log entries to that section instead of the default section.

Example configuration:

```yaml
section_header: "## 🕗"
section_header_work: "## 💼 Work"
section_header_personal: "## 🏠 Personal"
section_header_health: "## 🏥 Health"
```

So when you run `obsidian-logging -c work "Meeting"`, the entry will be logged under the "## 💼 Work" section. If the category doesn't have a corresponding section header defined, entries will be logged to the default section specified by `section_header`. 

## Environment variable 

If specified, $OBSIDIAN_VAULT_DIR will override the `vault` value in `obsidian-logging.yaml`


## Command line switches 

### No arguments 

When invoking `obsidian-logging` without any arguments the entries of the current day, if any, will be listed.  This is equivalent to `obsidian-logging -l`

### No switches

When invoking the command `obsidian-logging This is a log entry` obsidian-logging will append the string `This is a log entry` to the default log section of the markdown daily note. 
A timestamp will be prepended according to the chosen list mode. If list mode is `bullet`, '- HH:mm ' is prepended to the log statement.  If list mode is 'table', the log statement is 
wrapped in markdown table column separators:  `| HH:mm | log statement|`

## Usage Examples

```bash
# List today's entries
obsidian-logging

# Add a log entry to the default section
obsidian-logging "Had lunch with colleagues"

# Add an entry with a specific time
obsidian-logging -t 14:30 "Team standup meeting"

# Add entries to different categories
obsidian-logging -c work "Code review completed"
obsidian-logging -c personal "Gym workout - 45 minutes"
obsidian-logging -c health "Annual checkup scheduled"

# Combine category with time override
obsidian-logging -c work -t 9:00 "Daily standup"

# Read from stdin (useful for piping)
echo "Quick note" | obsidian-logging -S
cat notes.txt | obsidian-logging -c work -S

# List entries from previous days
obsidian-logging -b 1  # Yesterday's entries
obsidian-logging -b 7  # One week ago

# List entries by category
obsidian-logging -l -c work      # List only work entries
obsidian-logging -l -c personal  # List only personal entries
obsidian-logging -l -c work -c personal  # List work and personal entries
obsidian-logging -l -c all       # List entries from all categories
obsidian-logging -b 1 -c work    # List work entries from yesterday

# Edit today's file directly
obsidian-logging -e
```

### -t or --time 

The timestamp may be overridden by specifying the -t/--time HH:mm switch.  Log entries are sorted chronologically before being added to the markdown file. If using the 12 hour clock format (`time_format: 12` in the config file), the format is HH:mmA. For example `08:13PM`. 


### -l  or --list 

You can list the current days log entries by specifying the -l option.  If obsidian-logging is invoked without any arguments, this is the default action.

When combined with the `-c` or `--category` option, only entries from the specified category section will be listed. If no category-specific section header is found, entries from the default section will be shown. The `-c` option can be specified multiple times to list entries from multiple categories, or use `-c all` to list entries from all categories.

Examples:
```bash
obsidian-logging -l                    # List entries from default section
obsidian-logging -l -c work            # List entries from work category
obsidian-logging -l -c personal        # List entries from personal category
obsidian-logging -l -c work -c personal # List entries from work and personal categories
obsidian-logging -l -c all             # List entries from all categories
obsidian-logging -l -c unknown         # List entries from default section (fallback)
```

### -b <days> or --back <days>

By specifying `-b <number>` you can go back in time and list the logs `number` of days ago. `obsidian-logging -b 0` is the same as `obsidian-logging -l`.  `-b` can be combined with `-c`. 

### -e or --edit

Invokes $EDITOR with todays file.  Uses vim if $EDITOR is not set,

### -f or --time-format 

Specifies 12H or 24H time format.  24H is default.   Overrides `time_format` in obsidian-logging.yaml. Combining 12-hour and 24 hour timestamps when adding logs may yield unpredictable results. 

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

### -c or --category
Specifies a category for the log entry. The entry will be logged to a section identified by the `section_header_<category>` property in the configuration file. If no category-specific section header is found, the entry will be logged to the default section specified by `section_header`.

When used with the `-l` option, this flag can be specified multiple times to list entries from multiple categories. Use `-c all` to list entries from all categories.

Examples:
```bash
# Adding entries
obsidian-logging -c work "Team meeting at 2pm"
obsidian-logging -c personal "Gym workout"
obsidian-logging -c health "Doctor appointment"

# Listing entries
obsidian-logging -l -c work                    # List work entries only
obsidian-logging -l -c work -c personal        # List work and personal entries
obsidian-logging -l -c all                     # List entries from all categories
``` 


## Example Output

With the category functionality, your daily notes can be organized into different sections. Here's an example of what a daily note might look like:

```markdown
# 2024-01-15

## 🕗

* 08:30 Morning coffee
* 12:00 Lunch break

## 💼 Work

* 09:00 Daily standup meeting
* 10:30 Code review completed
* 14:00 Client presentation

## 🏠 Personal

* 18:00 Gym workout - 45 minutes
* 19:30 Dinner with family

## 🏥 Health

* 11:00 Doctor appointment
* 16:00 Picked up prescription
```

## Screenshots

### Bullet mode 

![image](https://github.com/user-attachments/assets/72c50c59-5185-4cb4-a871-473a8fd8b96f)

### Table mode 

![image](https://github.com/user-attachments/assets/ad3fe2c4-9a33-4272-a059-3d22617cef97)


# Contact info 

https://mas.to/@jantzten

https://bsky.app/profile/leif.jantzen.no
