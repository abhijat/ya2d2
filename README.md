### Yet Another to-do droid


A command line todo shell

Tasks are stored in [sled](https://github.com/spacejam/sled) for persistence.
A directory called .ya2d2 is created in your $HOME for storing the tasks. 

A file called .ya2d2_hist stores command history.

The shell currently supports four commands: 

* push [add entry into TODO list]
* pop [remove entry (using its id as the argument)]
* ls [list all entries]
* change-prompt

There is basic support for tab completion of commands. 

[Linefeed](https://github.com/murarth/linefeed) is used to provide the shell features.


##### Command line flags

* --no-color : disables colored output
