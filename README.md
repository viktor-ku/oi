# oi

> Timer app that treats you like a human, works great with dmenu and terminal

## General usage

```
oi mate can you please remind me to take my soup out in 10 mins
```

Or just:

```
oi 10 mins
```
 
It is completely up to you. The only difference is that when timer runs out
it will create a notification with the specified input. So, if you plan
on having multiple timers at the same time, you should probably add
some input to it.

## Keywords

There are some keywords to specify your timer delay (timeout). There are no
rules other than these keywords. 

The order of words and keywords is entirely up to you.

You've got **short** and **long** variants to specify delays. You can combine these two forms
in the same command, it's not like you have to stick with the only one.

It is **important** to note that when using long variant the keyword follows
(number, space, keyword) the number, but with short variant keyword is exactly after
the number (number, keyword).

### Short

- `h`
- `m`
- `s`

e.g.

```
oi short for seconds 1s
oi short for minutes 2m
oi short for hours 3h
oi you can also combine them 1s 2m 3h
oi or even repeat 1s 2s 3s 4s 1m 2m like I dunno why but you can
```

### Long 

The long form has several variants and were added for your convenience. Looks
like this, e.g:

```
oi 1 hour
oi I have only 23 mins 2 hours and 30 seconds
oi will have to leave in 10 minutes
oi call him back in 1 hr and 15 min
```

#### Hours

- `hours`
- `hour`
- `hrs`
- `hr`
- `h`

#### Minutes

- `minutes`
- `minute`
- `mins`
- `min`
- `m`

#### Seconds

- `seconds`
- `second`
- `secs`
- `sec`
- `s`

## Overflowing

You can specify e.g. `300 secs` and it will be expanded to 5 minutes instead, so you
don't have to stick to traditional thinking of durations if you like. You can write:

```
oi call her back in 90 mins
```

And you will have your timer in an hour and 30 minutes!

## Floating numbers

You can use `.` in your delays. For example, when you need to quickly specify
2 hours 30 minutes, but you don't want to write it down, you can do this:

```
oi remind about it in 2.5 hours
oi or in short form 2.5h
```

Or, possibly the shortest way to describe 30 minutes is:

```
oi .5h and I am out
```

