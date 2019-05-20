# timeout
Timeout commands on the cli

Usage:

`timeout [time limit in seconds as positive integer <= u64::MAX] command [space separated command args]`

Will stop at the sooner of the command's completion or the timeout.

Default timeout is 5s.

Currently the error messages from timeout can be lost, not sure why.
