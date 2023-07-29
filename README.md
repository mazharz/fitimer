# fitimer

A timer to keep fit and healthy whilst having a sedentary work.

## Simplicity

You are either working or taking a break. So we have those two states in the real world. I used to use pomodoro timers for this, but this is simpler and has a nicer interface for me.

## How to install

```bash
$ cargo install fitimer
```

## Features

- desktop notifications
- customizable colors

## Env vars

Fitimer is configurable through environment variables, you can put these in your `~/.bashrc` or `~/.zshrc`:

```bash
# These are gruvbox colors
FITIMER_COLOR_DARK="#282828"
FITIMER_COLOR_GRAY="#928374"
FITIMER_COLOR_LIGHT="#fbf1c7"
FITIMER_COLOR_RED="#cc241d"
FITIMER_COLOR_GREEN="#98971a"
FITIMER_COLOR_YELLOW="#d79921"
FITIMER_COLOR_BLUE="#458588"
FITIMER_COLOR_PURPLE="#b16286"
FITIMER_COLOR_AQUA="#689d6a"
FITIMER_COLOR_ORANGE="#d65d0e"
# Durations are in mins
FITIMER_DURATION_WORK=25
FITIMER_DURATION_REST=5
# How frequent app should get re-rendered
# increase this to reduce CPU load
FITIMER_TICK_RATE=1000
# Statistics log file
FITIMER_CONFIG_DIR=.config/fitimer
FITIMER_LOG_FILE=fitimer.log
# Date format used throughout the application
FITIMER_DATE_FORMAT="%Y-%m-%d %H:%M:%S %z"
```

## Todo

- [x] setup tui
- [x] add timer state
- [x] toggle timer
- [x] change timer state between work and rest
- [x] save progress into a file on toggle/change
- [x] make graph for statistics
- [ ] add help menu for shortcuts
- [ ] add settings page
- [ ] add option to disable graph (prevents setting into file too)
- [ ] add option to delete stat file content
