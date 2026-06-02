# Tips on using Multipass

Tips in addition to the ones mentioned in the repo's actual contents.


## Keep the date

With Ubuntu 26.04 LTS, `chrony` has taken over the time keeping.

As a side effect, you'll likely see the VM time dropping way behind, after sleeping the host.

This is due to the configuration:

<sub>`/etc/chrony/chrony.conf`</sub>

```
# Step the system clock instead of slewing it if the adjustment is larger than
# one second, but only in the first three clock updates.
makestep 1 3
```

Change this to:

```
makestep 1 -1
```

>After the change:
>
>```
>$ sudo systemctl restart chrony
>```

This means that `chrony` can "catch up" any number of times, not just 3.

To test:

```
$ timedatectl 
               Local time: Tue 2026-06-02 14:09:49 EEST
           Universal time: Tue 2026-06-02 11:09:49 UTC
                 RTC time: Tue 2026-06-02 11:09:49
                Time zone: Europe/Helsinki (EEST, +0300)
System clock synchronized: yes
              NTP service: active
          RTC in local TZ: no
```

### What if you don't?

Your host files will have real-world time stamps (edited within an IDE).

Your *output*, however, will always be ole (run within VM). You'll have endless, unnecessary recompilations!

Yuck.


## `~/.espressif` on the host (macOS)

The author has noticed that:

- it's good for IDE to have `~/.espressif` populated also on the host

Some suggestion:

1. Build some subproject within the IDE console

	- open terminal
	- `$ cargo build --release -vv` or the like

	This builds the `~/.espressif` folder, and takes around 5.4GB.
	
	What we have now are e.g. the headers that 
	
	// tbd. did we tho?