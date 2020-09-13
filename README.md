# oi

> Timer app that treats you like a human, works great with dmenu and terminal

## General usage

Two mods are available to set up the timer: using 
[timeout](#timeout) _and_/_or_ using [exact time](#exact-time). 
It's not either-or, feel free to mix everything, even multiple 
times during the same command.

Order of timeouts, exact times and custom text is not important at all, 
you are free to specify the timer however you want to!

Quick example for the _timeout_:

```
oi pause in 3 min
```

Quick example for the _exact time_:

```
oi I should leave at 6pm
```

### To Message, Or not to Message
 
Message is completely optional - the only difference is that when timer 
runs out it will create a notification with the entire input. So, 
if you plan on having multiple timers at the same time, it might be 
useful to have message specifying what it was.

So, the following timers are exactly the same:

```
oi 4 minutes
oi remind me that I have a tea in the kitchen in 4 minutes
```

### Learn by Example

```
oi remind me to join the meeting call at 10:00
oi 1m 30s 45s 2 minutes .1h is exactly how much time I need after 2pm
```

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

```
oi 10h
oi 10 hr
oi 10 hours
```

### Minutes

- `minutes`
- `minute`
- `mins`
- `min`
- `m`

E.g.

```
oi 10m
oi 10 min
oi 10 minutes
```

### Seconds

- `seconds`
- `second`
- `secs`
- `sec`
- `s`

E.g.

```
oi 10s
oi 10 sec
oi 10 seconds
```

### Overflowing

You can specify e.g. `300 secs` and it will be expanded to 5 minutes instead, so you
don't have to stick to traditional thinking of durations if you like. You can write:

```
oi call her back in 90 mins
```

And you will have your timer in an hour and 30 minutes!

### Floating numbers

You can use `.` in your delays. For example, when you need to quickly specify
2 hours 30 minutes, but you don't want to write it down, you can do this:

```
oi the long and boring way 2 hours 30 minutes
oi slightly shorter but still boring way 2h 30m
oi or in short 2.5h
```

Or, possibly the shortest way to describe 30 minutes is:

```
oi .5h and it will be done
```

## Exact Time

Used to specify some time of interest explicitly. Under the hood it 
calculates the duration between the local _now_ and local time and 
launches an ordinary timeout:

```
oi should finish at 19:30
oi should finish at 7:30pm
```

Just like with timeouts, the space between the time and "am"/"pm"
is optional.

```
oi at 1am
oi at 1 am
```

Minutes part is optional and is going to be set to `0` by default, so
the following timers are equal:

```bash
oi at 22
oi at 22:00
oi at 10pm
oi at 10:00 pm
```

### Where a day starts and ends?

In case specified time is in the past relative to the current 24h day, 
it carries out to the next day, for example:

```bash
# Imagine it is October 1st, 11:30pm (23:30)
oi definitely go to sleep at 2am
```

There already was point in time when _2am_ of October 1st occurred, so
it carries out to the next day, October the 2nd, effectively setting your
timer for 2 hours 30 minutes.

The same happens with more distant points in time, like this:

```bash
# Imagine it is October 1st, 4:15pm (16:15)
oi at 4:15am
```

Sets your timer up for 12 hours.
