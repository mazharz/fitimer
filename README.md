# fitimer

A timer to keep fit and healthy whilst having a sedentary working.

## Simplicity

You are either working or taking a break. So we have those two states in the real world. I used to use pomodoro timers for this, but this is simpler and has a nicer interface IMO.

## How it works

This is a TUI application with vim-like keybindigns.

## Todo

- [x] setup tui
- [x] add tabs
- [x] refactor main file
- [x] replace all unwraps with expect with good error messages
- [x] add timer state
- [x] toggle timer
- [ ] change timer state between work and rest
- [ ] extend break

## Env vars

```bash
# These are gruvbox colors (all in hex code)
COLOR_DARK="#282828"
COLOR_GRAY="#928374"
COLOR_LIGHT="#fbf1c7"
COLOR_RED="#cc241d"
COLOR_GREEN="#98971a"
COLOR_YELLOW="#d79921"
COLOR_BLUE="#458588"
COLOR_PURPLE="#b16286"
COLOR_AQUA="#689d6a"
COLOR_ORANGE="#d65d0e"
# Durations are in mins
DURATION_WORK=25
DURATION_REST=5
DURATION_EXTEND=5
# How frequent app should get re-rendered
TICK_RATE=250
```

## Features

- [ ] notification
- [ ] customizable colors
