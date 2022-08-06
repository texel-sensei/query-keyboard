# query-keyboard

Simple script to query which keys are hold on the keyboard.

Returns 0 if all keys specified on the command line are hold and 1 otherwise.

Requires libx11-dev on Linux.

The `--hold-for` parameter can be used to specify a time in seconds for which the keys need to be
held.
If a key is released before this time has expired, then the script will return 1.

## Example

    query-keyboard LShift A

	query-keyboard --hold-for 0.2 Enter
