# fitimer

A timer to keep fit and healthy whilst having a sedentary working.

# Simplicity

You are either working or taking a break. So we have those two states in the real world. I used to use pomodoro timers for this, but this is simpler and has a nicer interface IMO.

# How it works

This is a server listening on a port of your choosing. What about client? We have `curl` already, and HTTP is more than capable of handling all our usecases.

# Endpoints

- [ ] start timer
- [ ] stop timer
- [ ] toggle timer
- [ ] extend break

# Env vars

- WORK_DURATION (in mins)
- REST_DURATION (in mins)
- EXTEND_DURATION (in mins)

# Features

- [ ] notification
