1. Run `just build-components` to build the js and rust components
2. Run `just run (js|rs)` to measure memory usage of running js/rs component.

Output: rust component has constant RES memory:

```
$ just run rs

Iteration: 0, Virtual: 4822764 KB, RES: 17560 KB
Iteration: 100000, Virtual: 4823024 KB, RES: 17560 KB
Iteration: 200000, Virtual: 4823024 KB, RES: 17560 KB
Iteration: 300000, Virtual: 4823024 KB, RES: 17560 KB
Iteration: 400000, Virtual: 4823024 KB, RES: 17560 KB
Iteration: 500000, Virtual: 4823024 KB, RES: 17560 KB
Iteration: 600000, Virtual: 4823024 KB, RES: 17560 KB
Iteration: 700000, Virtual: 4823024 KB, RES: 17560 KB
Iteration: 800000, Virtual: 4823024 KB, RES: 17560 KB
Iteration: 900000, Virtual: 4823024 KB, RES: 17560 KB
...
```

whereas jco component increases over time and shows no indication of stabilizing:
```
$ just run js

Iteration: 0, Virtual: 4900460 KB, RES: 389648 KB
Iteration: 100000, Virtual: 4900720 KB, RES: 421924 KB
Iteration: 200000, Virtual: 4900720 KB, RES: 427492 KB
Iteration: 300000, Virtual: 4900720 KB, RES: 428812 KB
Iteration: 400000, Virtual: 4900720 KB, RES: 430396 KB
Iteration: 500000, Virtual: 4900720 KB, RES: 432240 KB
Iteration: 600000, Virtual: 4900720 KB, RES: 434088 KB
Iteration: 700000, Virtual: 4900720 KB, RES: 435672 KB
Iteration: 800000, Virtual: 4900720 KB, RES: 436992 KB
Iteration: 900000, Virtual: 4900720 KB, RES: 438312 KB
Iteration: 1000000, Virtual: 4900720 KB, RES: 439632 KB
...
Iteration: 6700000, Virtual: 4900720 KB, RES: 528600 KB
..
etc...
```