### Setting up Raspberry Pi

I've started messing about with a RPI4, intending to run realtime code on one or two cpus using partrt.

First minor glitch was installing partrt.  I ended up cd'ing into the bincalc and partrt directories, and running make install in each directory.  I probably just don't understand cmake well enough.

Second minor glitch was that partrt was erroring because of behavior of sh.  I changed the header to #!/bin/bash -eu and then it works fine.

I may try using rust with the rpal crate.  Stay tuned.

This looks useful: https://raspberrypi.stackexchange.com/questions/40105/access-gpio-pins-without-root-no-access-to-dev-mem-try-running-as-root

For the purpose of bell swing detection, I'm expecting pulses of about 10msec in duration.  This means we will want a rise time of about 1msec.  Cat3 cable has a capacitance of roughly 18 pF / foot.  If the cables to the bells are roughly 40 ft long, that means about 720 pF, and we want rise time of 1msec, that means R should be around 1.5kohm.  [I currently have 4.7kohm resistors available, so I expect in the short term to see rise time of around 3 msec, which is likely tolerable].

Bluetooth pairing:
https://bluedot.readthedocs.io/en/latest/pairpipi.html#using-the-command-line
Then use the connect MACADDRESS after pairing.

aplay can play a wav file, and concurrent aplay will be mixed together!

## Rust
The default rust install is 1.63, which is old enough to cause problems, e.g. with eframe.
Rustup is only available through snap, not through apt, so I installed that, then installed rustup.
Currently, though rustup command is not found.

