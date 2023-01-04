# boilerBOT
boilerBOT is a discord bot that does nothing of note. It's purpose is to provide a boilerplate/library that can be used to make quick, useful bots using the nextcord library without having to redo basics like command structure, embed styling, configuration, etc.

A lot of boilerBOT's code will be ripped from my older bot projects, meaning they should be pretty straightforward to port over as examples once boilerBOT is fuctional.

To use boilerBOT, put this repo somewhere in your python path (e.g `~/.local/lib/python3.10/site-packages/`) and then 
```python 
import boilerBot
```
The bot will pull configuration variables from a `config.yaml` stored in the root directory of the project. Put your token and other things in there.

See TEMPLATES.md for folder layout, command structure, etc guides.