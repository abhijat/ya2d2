### Yet Another to-do droid


[![Build Status](https://travis-ci.com/abhijat/ya2d2.svg?branch=master)](https://travis-ci.com/abhijat/ya2d2)


###### A shell to store and interact with Todo entries

[![asciicast demo](https://asciinema.org/a/0XKCD4XjpLOUaublHTwViNadN.svg)](https://asciinema.org/a/0XKCD4XjpLOUaublHTwViNadN)

Tasks are stored in [sled](https://github.com/spacejam/sled) for persistence.

Files created:
* $HOME/.ya2d2 [a directory containing saved ToDo entries]
* $HOME/.ya2d2_hist [Command history to preserve commands across sessions]


The shell currently supports four commands: 

* push [add entry into TODO list]
* pop [remove entry (using its id as the argument)]
* ls [list all entries]
* ~~change-prompt~~ (change-prompt has not been incorporated into the nom parsing structure yet. )


There is basic support for tab completion of commands as well as the usual readline features. 
For example, press tab after "pop" to view the keys for entries which are present in the database.

These features are provided by hooking into the excellent [Linefeed](https://github.com/murarth/linefeed) library.


##### Command line flags

* --no-color : disables colored output

##### OS compatibility

The application is developed on Linux, but it should also work on OSX.

##### Todo
* Since moving the parsing infrastructure to nom, there is now support for tags, eg the following command will
parse and recognize "grocery" and "budget" as tags.  

```$xslt
push [[grocery]] [[budget]] buy cheese
```
But the backend does not actually save these tags right now. Need to store these along with entries so listing can work with tags.

* change-prompt was removed when moving to nom. Need to write supporting infrastructure to add this back.
* Support listing specific tags, eg
```$xslt
ls grocery
```

* add support to export the database to json
