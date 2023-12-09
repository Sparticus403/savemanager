# savemanager
CLI program to backup, revert, and swap save files for videogames.

## Please Note:
This program currently works only with minecraft worlds running on Linux systems. Implementation on other game files and operating systems will come in later versions.

## Building The Project:
Before building the project, first navigate inside the savemanager directory, and build from inside. This must be done since the project itself is located within this directory. Once inside the `savemanager` directory, build as you would any other rust project. 

## Using the Product
To use the savemanager, execute it with `./ds3_swapper`, and pass in `-r`, `-b`, or `-s` followed by the name of the minecraft world which you would like to revert, backup, or swap. Additonally, the flag `-h` can be used to display help text.
Ex. running `./ds3_swapper -b 'New World'` would create a backup to a minecraft world titled 'New World'. It is important to note that should a save have more than one word in the title, that single quotes (i.e. `'`) surround the name of the save. 
