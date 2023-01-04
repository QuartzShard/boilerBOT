# boilerBOT
boilerBOT is a discord bot that does nothing of note. 
It's purpose is to provide a quick and easy starting point to write a discord bot, without having to redo command registration boilerplate and other such things. It also supplies some utility funtions for styling embeds.

### boilerBOT is currently undergoing a rust-rewrite on this branch `oxidation`. Features will be missing for a while.

To use boilerBOT, clone the repo and change the name of the crate in `Cargo.toml`.
The bot will pull configuration variables from a `config.toml` stored in the root directory of the project. Put your token and other things in there. If you'd rather use environment varialbes, a method will be provided for building the config struct from `std::env`, just drop that in in place of the toml parser.

A ping command will be provided as a simple example. Other commands are planned, too, such as the auto-generating help from the python version.
