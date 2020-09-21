# oi

> Timer app that treats you like a human, works great with dmenu and terminal

## General usage

Use `run` subcommand followed by a timer definition to launch a timer.

Two mods are available to set up the timer: using 
[timeout](#timeout) _and_/_or_ using [exact time](#exact-time). 
It's not either-or, feel free to mix everything, even multiple 
times during the same command.

Order of timeouts, exact times and custom text is not important at all, 
you are free to specify the timer however you want to!

Quick example for the _timeout_:

```bash
$ oi run "pause in 3 min"
```

Quick example for the _exact time_:

```bash
$ oi run "I should leave at 6pm"
```

In the latter, it will calculate duration between your local "now" and 
specified _exact time_.

#### To Message, Or not to Message
 
Message is completely optional - the only difference is that when timer 
runs out it will create a notification with the entire input. So, 
if you plan on having multiple timers at the same time, it might be 
useful to have message specifying what it was.

So, the following timers are exactly the same, all three of them will launch
a timer for 4 minutes.

```bash
$ oi run "4 minutes"
$ oi run "4m" # You have different methods to write the length (See below for more)
$ oi run "remind me that I have a tea in the kitchen in 4 min"
```

### Learn by Example

#### The simplest timer you can launch

```bash
$ oi run "my first timer ever for 30 seconds"
```

_This will launch a timer for 30 seconds!_

#### You can combine multiple durations

These durations might be of any length as long as it makes sense to you!

```bash
$ oi run "1 minute 30s .5m 3600 seconds"
```

_This will launch a timer for an hour and 2 minutes!_

#### Exact times can be used

duration between current time and specified value will be
calculated and added:

_consider it is 8 am now (08:00)_

```bash
$ oi run "remind me to join the meeting call at 9:15"
```

_This will launch a timer for an hour and 15 minutes!_

#### You can use `-` in front of the duration 

It will tell the timer that you want to move the target one hour towards your current time:

```bash
$ oi run "2h -1h -30m"
```

_This will launch a timer for 30 minutes!_

##### It works great when using with exact times

_consider it is 4pm now (16:00)_

```bash
$ oi run "originally event starts at 6pm was moved 1 hour I need -2 hours to get there"
```

_This will launch a timer for an hour!_

## Timeout

Timer is a total of all specified timeouts combined. You can specify
timeouts using keywords for _hours_, _minutes_ and _seconds_.

_Note_ that the space between the timeout and keyword is optional.

### Hours

- `hours`
- `hour`
- `hrs`
- `hr`
- `h`

E.g.

```bash
$ oi run 10h # You can leave out quotes when using just one timeout
$ oi run "10 hr"
$ oi run "10 hours"
```

### Minutes

- `minutes`
- `minute`
- `mins`
- `min`
- `m`

E.g.

```bash
$ oi run 10m
$ oi run "10 min"
$ oi run "10 minutes"
```

### Seconds

- `seconds`
- `second`
- `secs`
- `sec`
- `s`

E.g.

```bash
$ oi run 10s
$ oi run "10 sec"
$ oi run "10 seconds"
```

### Floating numbers

You can use `.` in your timeouts. For example, when you need to quickly specify
2 hours 30 minutes, but you don't want to write it down, you can do this:

```bash
$ oi run "the long and boring way 2 hours 30 minutes"
$ oi run "slightly shorter but still boring way 2h 30m"
$ oi run "or in short 2.5h"
```

Or, possibly the shortest way to describe 30 minutes is:

```bash
$ oi run ".5h and it will be done"
```

## Exact Time

Used to specify some time of interest explicitly. Under the hood it 
calculates the duration between the local _now_ and local time and 
launches an ordinary timeout:

```bash
$ oi run "should finish at 19:30"
$ oi run "should finish at 7:30pm"
```

Just like with timeouts, the space between the time and "am"/"pm"
is optional.

```bash
$ oi run "at 1am"
$ oi run "at 1 am"
```

Minutes part is optional and is going to be set to `0` by default, so
the following timers are equal:

```bash
$ oi run "at 22"
$ oi run "at 22:00"
$ oi run "at 10pm"
$ oi run "at 10:00 pm"
```

### Where a day starts and ends?

In case specified time is in the past relative to the current 24h day, 
it carries out to the next day, for example:

_Consider it is October 1st, 11:30pm (23:30)_

```bash
$ oi run "definitely go to sleep at 2am"
```

There already was point in time when _2am_ of October 1st occurred, so
it carries out to the next day, October the 2nd, effectively setting your
timer for 2 hours 30 minutes.

The same happens with more distant points in time, like this:

_consider it is October 1st, 4:15pm (16:15)_

```bash
oi run "at 4:15am"
```

Sets your timer up for 12 hours.
