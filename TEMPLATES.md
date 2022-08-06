## Project Structure

After moving boilerBOT's code to somewhere on your python path, you need to set up your bot's environment. The things you'll need are:
* A main .py file for you propject (Often bot.py)
* A config.yaml in the same folder, using the structure found in config-example.yaml. You'll need to create a bot and provide the login token in this file, see [the discord developer page](https://discord.com/developers/applications)
* A subfolder called `commands/`, where you will store the command logic for your bot. Optionally, you can create categorised subfolders in here for your commands.
* *OPTIONAL*: A subfolder called `common/`, where you can store code you intend to re-use in multiple places. By writing an `__init__.py`, you can make these easier to import. 

A typical project folder will look something like this:
```
examplebot/
├─ commands/
│  ├─ fun/
│  │  └─ telljoke.py
│  ├─ utility/
│  │  ├─ telltime.py
│  │  └─ colourrole.py
│  ├─ moderation/
│  │  ├─ muteuser.py
│  │  └─ banuser.py
├─ common/
│  ├─ __init__.py
│  └─ utilityfunc.py
├─ bot.py
└─ config.yaml

```

## Main bot.py
The bare minimum to initialise boilerBOT is this:
```python
import boilerBot as bb # or whatever name you prefer

botClient = bb.bot(bb.lib.cfg['options']['prefix'], intents=bb.intents)
bb.atexit.register(botClient.shutdown)
botClient.run(bb.lib.cfg['discord']['token'])
```
If you need to store per-server information with your bot, you should specify the default state of `guildVars` when you initialise. This is a guild-id keyed `dict` that will hold your variables.
```python
import boilerBot as bb # or whatever name you prefer

botClient = bb.bot(bb.lib.cfg['options']['prefix'], defaultGuildVars={'var1':False,'var2':[],'var3':None}, intents=bb.intents)
bb.atexit.register(botClient.shutdown)
botClient.run(bb.lib.cfg['discord']['token'])
```
If you need to modify any of the core behaviour of the bot.py, you can inherit it like so and override its' core funcitons: ~~but at this point, why not write the whole thing from scratch?~~
```python
import boilerBot as bb

class myBot(bb.bot):
    def __init__(self, *args, **kwargs):
        ## Make sure upstream bot stuff gets done
        super().__init__(*args,**kwargs) 
    
    ## Modifications go here!

botClient = myBot(bb.lib.cfg['options']['prefix'], intents=bb.intents)
...

```



## Commands
Make a `commands/` directory at the root of your bot's project folder (where the bot.py is).
Populate is with .py files, 1 per command, optionally organised into category subfolders (e.g `moderation/`,`fun/`,`music/`). The help command will recognise these folder names and gorup commands accordingly.
Structure the files like so:

```python
## Initialisation
import boilerBot.lib as lib
import discord

from discord.ext import commands, tasks

## Define command cog
class COMMAND_NAME(commands.Cog):
    ## Initialise with help info
    def __init__(self,bot):
        self.bot = bot
        self.category = lib.getCategory(self.__module__)
        self.description = "COMMAND DESCRIPTION"
        self.usage = f"""
        {self.bot.command_prefix}COMMAND NAME + USAGE INSTRUCTIONS
        """
        self.forbidden = False
        
    @commands.command()
    async def COMMAND_NAME(self, ctx, *command):
        return ctx.send("Actual command code goes here")
    
def setup(bot):
    bot.add_cog(COMMAND_NAME(bot))
```
As an example, here is the built-in about command:
```python
## Initialisation
import boilerBot.lib as lib
import discord

from discord.ext import commands, tasks

## Define about cog
class about(commands.Cog):
    ## Initialise with help info
    def __init__(self,bot):
        self.bot = bot
        self.category = lib.getCategory(self.__module__)
        self.description = f"Display information about {self.bot.user.name}"
        self.usage = f"""
        {self.bot.command_prefix}about
        """
        self.forbidden = False
        
    ## Callable command to provide info about bot
    @commands.command()
    async def about(self, ctx, *command):
        embed=lib.embed(
            title=lib.cfg['about']['title'],
            description=lib.cfg['about']['desc'],
            url=lib.cfg['about']['url'],
            thumbnail=True
        )
        await ctx.send(embed=embed)
    
def setup(bot):
    bot.add_cog(about(bot))

```

## Common
Common code can be stored seperate to commands and, with the use of an `__init__.py`, easilly imported to other files in the project. 
The simplest `__init__.py` consists of:
```python
from . import *
```
Which allows you to pull in classes, functions and variables from files in `common/` anywhere else in your project like so:
```python
from common import utilityFunc, TemplateClass #etc
```
This can greatly improve consistence accross your project and reduce re-writing of code, and is essentially how boilerBOT started.
