# savemanager
CLI program to backup, switch out, and create new save files for videogames

As it stands now, this program cannot perform advertised functions. That is still a work in progress.
This program can create a directory for games, and will store a JSON file that will store information about the game that's passed in as an argument.

## Using the Product
To use the savemanager, execute it as you would any cli app, and pass in `-c`, `-b`, or `-s` followed by the name of a game in which you would like to create, backup, or swap save files. 
Currently only the `-c` flag has any kind of implementation to it, but it is not yet able to be used to create new game save files as is intended. It does, however, create a local directory, in which it will be able to store game information, such as the filepath to the save file directory.
