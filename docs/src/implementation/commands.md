# Commands and subprocesses

Commands and subprocesses are divided into two classes: short-lived and
long-lived jobs

## Short-lived jobs
These are jobs that perform only one task and are terminated as soon as they
finished, so only one can be ran at a time.

While these jobs are running, only the SigTerm [message](./messages.md) should
be handled, which sends a SIGTERM signal to it.

At the same time, this message should only be available when a *short-lived job*
is running

Examples of **short-lived jobs** are:
- ssh-keygen, used when adding a new key
- ssh-add, used when adding a new key to the ssh agent

## Long-lived jobs
These are jobs intended to run in the background of the main process.
