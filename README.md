[![Build Status](https://travis-ci.com/abhijat/ya2d2.svg?branch=master)](https://travis-ci.com/abhijat/ya2d2)

### Yet Another to-do droid


A command line todo shell

Tasks are stored in [sled](https://github.com/spacejam/sled) for persistence.

Files created:
* $HOME/.ya2d2 [a directory containing saved ToDo entries]
* $HOME/.ya2d2_hist [Command history to preserve commands across sessions]


The shell currently supports four commands: 

* push [add entry into TODO list]
* pop [remove entry (using its id as the argument)]
* ls [list all entries]
* change-prompt


There is basic support for tab completion of commands as well as the usual readline features. 

These features are provided by the excellent [Linefeed](https://github.com/murarth/linefeed) library.


##### Command line flags

* --no-color : disables colored output

##### OS compatibility

The application is developed on Linux, but it should also work on OSX. 
