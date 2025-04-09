### Setting up Raspberry Pi

I've started messing about with a RPI4, intending to run realtime code on one or two cpus using partrt.

First minor glitch was installing partrt.  I ended up cd'ing into the bincalc and partrt directories, and running make install in each directory.  I probably just don't understand cmake well enough.

Second minor glitch was that partrt was erroring because of behavior of sh.  I changed the header to #!/bin/bash -eu and then it works fine.

I may try using rust with the rpal crate.  Stay tuned.

This looks useful: https://raspberrypi.stackexchange.com/questions/40105/access-gpio-pins-without-root-no-access-to-dev-mem-try-running-as-root
