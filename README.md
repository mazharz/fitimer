# fitimer

A timer to keep fit and healthy whilst having a sedentary working.

## Simplicity

You are either working or taking a break. So we have those two states in the real world. I used to use pomodoro timers for this, but this is simpler and has a nicer interface IMO.

## How it works

This is a TUI application with vim-like keybindigns.

## Todo

- [x] setup tui
- [ ] refactor main file
- [ ] replace all unwraps with expect with good error messages
- [ ] start timer
- [ ] stop timer
- [ ] toggle timer
- [ ] extend break

## Env vars

```bash
# These are gruvbox colors
COLOR_DARK="#282828" # hex color code
COLOR_GRAY="#928374" # hex color code
COLOR_LIGHT="#fbf1c7" # hex color code
COLOR_RED="#cc241d" # hex color code
COLOR_GREEN="#98971a" # hex color code
COLOR_YELLOW="#d79921" # hex color code
COLOR_BLUE="#458588" # hex color code
COLOR_PURPLE="#b16286" # hex color code
COLOR_AQUA="#689d6a" # hex color code
COLOR_ORANGE="#d65d0e" # hex color code
WORK_DURATION=25 # in mins
REST_DURATION=5 # in mins
EXTEND_DURATION=5 # in mins
```

## Features

- [ ] notification
- [ ] customizable colors
