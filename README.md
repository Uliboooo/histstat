# histstat

`histstat` is a small CLI tool that analyzes your shell history file and shows a ranking of the most frequently executed **successful commands** (exit status = 0).

It automatically detects the latest history file from **bash**, **zsh**, or **fish**, so no manual configuration is required.

## Features

- Automatically detects bash / zsh / fish history files
- Selects the most recently modified history file
- Counts only successful commands (exit status = 0)
- Handles `sudo` correctly (counts the actual command, not `sudo`)
- Sorts commands by execution count
- Configurable output size via CLI option

## Usage

Show the top 10 commands (default).
```sh
histstat
```

Show the top 5 commands.
```
histstat -c 5
```
