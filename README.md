# Obsidian logging - Log to Obsidian Journal from command line 

[![Crates.io](https://img.shields.io/crates/v/obsidian-logging.svg)](https://crates.io/crates/obsidian-logging)
[![Documentation](https://docs.rs/obsidian-logging/badge.svg)](https://docs.rs/obsidian-logging)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This little utility written in Rust makes it possible to log directly to todays daily note from the linux command line. 

## Version 1.3.0 Changes - Timestamp Format Update

**Important:** Starting with version 1.3.0, timestamps now include seconds (HH:mm:ss format instead of HH:mm).

**What this means for you:**
- New entries will have timestamps with seconds (e.g., `14:30:45` instead of `14:30`)
- When using the `-t` flag, you can specify seconds: `-t 14:30:45` or omit them: `-t 14:30` (defaults to `:00`)
- **Existing log files:** When you add a new entry to a file, all existing timestamps in that log section will be automatically reformatted to include seconds (e.g., `14:30` becomes `14:30:00`)
- Duplicate timestamp detection: If you add an entry with a timestamp that already exists, the seconds will be incremented until a unique timestamp is found

This change ensures better precision and prevents timestamp collisions when logging multiple entries at the same minute.

## License 

This software is licensed under a combined MIT and SPPL license.  It is basically a MIT license, but in order to be compliant you need to send me a postcard.  Details in LICENSE.md

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

### Predefined Phrases

You can define common logging phrases in your configuration file to use with the `-p` or `--phrase` option. This allows you to create shorthand references for frequently used log entries.

**Basic phrase configuration:**
```yaml
phrases:
  meeting: "Team meeting with stakeholders"
  gym: "Workout at the gym"
  lunch: "Lunch break"
  doctor: "Doctor appointment"
```

**Phrases with argument expansion:**
```yaml
# Conjunction for {#} placeholder is automatically selected based on locale
# Supported locales: no/nb/nn (og), da (og), sv (och), de (und), fr (et), es (y), it (e), pt (e), ru (и), ja (と), ko (와), zh (和)
# Defaults to "and" for English or unsupported locales

phrases:
  # Simple phrases
  meeting: "Team meeting with stakeholders"
  gym: "Workout at the gym"
  
  # Phrases with argument expansion
  meeting_with: "Team meeting with {*}"           # All arguments
  call_with: "Phone call with {0}"               # First argument only
  project: "Working on {0}"                     # First argument only
  exercise: "Exercise: {*}"                     # All arguments
  travel: "Travel to {0}"                       # First argument only
  food: "Ate {*}"                               # All arguments
  
  # Phrases with {#} placeholder for comma-separated lists
  meeting_with: "Team meeting with {#}"          # "John and Jane" or "John, Jane and Bob"
  call_with: "Phone call with {#}"              # "Alice and Bob" or "Alice, Bob and Charlie"
  project_with: "Working on {#}"                # "Frontend and Backend"
  exercise_with: "Exercise: {#}"                # "Running and Swimming"
```

**Usage examples:**
```bash
# Basic phrases
obsidian-logging -p meeting
obsidian-logging -p gym -c health

# Phrases with arguments
obsidian-logging -p meeting_with John Smith     # "Team meeting with John Smith"
obsidian-logging -p call_with Alice            # "Phone call with Alice"
obsidian-logging -p project "Project Alpha"   # "Working on Project Alpha"
obsidian-logging -p exercise "Running 5km"    # "Exercise: Running 5km"

# Phrases with {#} placeholder for comma-separated lists
obsidian-logging -p meeting_with John Jane        # "Team meeting with John and Jane"
obsidian-logging -p call_with Alice Bob Charlie  # "Phone call with Alice, Bob and Charlie"
obsidian-logging -p project_with Frontend Backend # "Working on Frontend and Backend"
``` 

## Environment variable 

If specified, $OBSIDIAN_VAULT_DIR will override the `vault` value in `obsidian-logging.yaml`


## Command line switches 

### No arguments 

When invoking `obsidian-logging` without any arguments the entries of the current day, if any, will be listed.  This is equivalent to `obsidian-logging -l`

### No switches

When invoking the command `obsidian-logging This is a log entry` obsidian-logging will append the string `This is a log entry` to the default log section of the markdown daily note. 
A timestamp will be prepended according to the chosen list mode. If list mode is `bullet`, '- HH:mm:ss ' is prepended to the log statement (e.g., `- 14:30:45 log entry`).  If list mode is 'table', the log statement is 
wrapped in markdown table column separators:  `| HH:mm:ss | log statement|` (e.g., `| 14:30:45 | log entry |`).

**Note:** Timestamps now include seconds (HH:mm:ss format). When you add a new entry, all existing entries in that log section will be reformatted to include seconds if they don't already have them. This ensures consistency across all entries.

## Usage Examples

```bash
# List today's entries
obsidian-logging

# Add a log entry to the default section
obsidian-logging "Had lunch with colleagues"

# Add an entry with a specific time (seconds default to 00 if not provided)
obsidian-logging -t 14:30 "Team standup meeting"        # Becomes 14:30:00
obsidian-logging -t 14:30:45 "Team standup meeting"     # Explicit seconds

# 12-hour format examples
obsidian-logging -t 2:30 PM "Afternoon meeting"         # Becomes 02:30:00 PM
obsidian-logging -t 2:30:45 PM "Afternoon meeting"      # Explicit seconds

# Add entries to different categories
obsidian-logging -c work "Code review completed"
obsidian-logging -c personal "Gym workout - 45 minutes"
obsidian-logging -c health "Annual checkup scheduled"

# Combine category with time override
obsidian-logging -c work -t 9:00 "Daily standup"

# Use predefined phrases
obsidian-logging -p meeting                    # Basic phrase
obsidian-logging -p gym -c health             # Phrase with category
obsidian-logging -p meeting_with John Smith   # Phrase with arguments
obsidian-logging -p call_with Alice -t 14:30 # Phrase with arguments and time

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

The timestamp may be overridden by specifying the `-t/--time` switch. You can provide timestamps in either `HH:mm` or `HH:mm:ss` format. If seconds are not provided, they default to `00`.

**Format examples:**
- 24-hour format: `14:30` (becomes `14:30:00`) or `14:30:45`
- 12-hour format: `2:30 PM` (becomes `02:30:00 PM`) or `2:30:45 PM`

Log entries are sorted chronologically before being added to the markdown file. If a timestamp already exists in the log, the seconds will be incremented until a unique timestamp is found.

**Important:** When you add a new entry, all existing entries in that log section will be reformatted to include seconds (HH:mm:ss format) if they don't already have them. This ensures consistency but means existing timestamps without seconds will be modified. 


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

### -p or --phrase
Use a predefined phrase from the configuration file with optional argument expansion. This allows you to define common logging phrases in your config and reference them with shorthand.

**Argument Expansion Support:**
- `{0}`, `{1}`, `{2}`, etc. - Replace with specific argument by index
- `{*}` - Replace with all arguments joined by spaces
- `{#}` - Replace with all arguments in comma-separated list with proper conjunction

Examples:
```bash
# Basic phrase usage (no arguments)
obsidian-logging -p meeting
obsidian-logging -p gym -c health

# Phrase with argument expansion
obsidian-logging -p meeting_with John Smith        # "Team meeting with John Smith"
obsidian-logging -p call_with Alice               # "Phone call with Alice"
obsidian-logging -p project "Project Alpha"      # "Working on Project Alpha"

# Phrase with {#} placeholder for comma-separated lists
obsidian-logging -p meeting_with John Jane        # "Team meeting with John and Jane"
obsidian-logging -p call_with Alice Bob Charlie  # "Phone call with Alice, Bob and Charlie"
obsidian-logging -p project_with Frontend Backend # "Working on Frontend and Backend"

# Combine with other options
obsidian-logging -p meeting_with John -t 14:30    # With specific time
obsidian-logging -p doctor_with "Dr. Smith" -c health  # With category
``` 


## Example Output

With the category functionality, your daily notes can be organized into different sections. Here's an example of what a daily note might look like:

```markdown
# 2024-01-15

## 🕗

* 08:30:00 Morning coffee
* 12:00:00 Lunch break

## 💼 Work

* 09:00:00 Daily standup meeting
* 10:30:00 Code review completed
* 14:00:00 Client presentation

## 🏠 Personal

* 18:00:00 Gym workout - 45 minutes
* 19:30:00 Dinner with family

## 🏥 Health

* 11:00:00 Doctor appointment
* 16:00:00 Picked up prescription
```

**Note:** All timestamps now include seconds. If you have existing log files with timestamps in `HH:mm` format, they will be automatically reformatted to `HH:mm:ss` format (with `:00` seconds) when you add a new entry to that file.

## Screenshots

### Bullet mode 

![image](https://github.com/user-attachments/assets/72c50c59-5185-4cb4-a871-473a8fd8b96f)

### Table mode 

![image](https://github.com/user-attachments/assets/ad3fe2c4-9a33-4272-a059-3d22617cef97)


# Contact info 

https://mas.to/@jantzten

https://bsky.app/profile/leif.jantzen.no
