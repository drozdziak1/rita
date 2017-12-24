# Building
This repository relies on [git
submodules](https://git-scm.com/book/en/v2/Git-Tools-Submodules). To pull
everything and build, run:
```sh
$ ./submodules.sh
$ cargo build
```

**Note: even though `submodules.sh` will not touch unstaged or uncommitted
subchanges, it might rewind each submodule to the point chosen by the Rita repo.
Keep this in mind when doing ad-hoc tests on modified dependencies.**

# Original README
the daemon calls ip neighbor to get active neighbors.

it then calls babeld to get destinations and prices.

then it sets ebtables rules tracking every neighbors traffic to every
destination.

then it waits a bit.

then it calls ebtables to get every neighbors traffic to every destination.

then it crunches this info down with the prices to get a tally of what each
neighbor owes.

then it adds this to the neighbors overall tally.

then it starts the first part of the cycle again
