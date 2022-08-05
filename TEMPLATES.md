## Main bot.py
The bare minimum to initialise boilerBOT is this:
```python
import boilerBot as bb # or whatever name you prefer

botClient = bb.bot(bb.lib.cfg['options']['prefix'], intents=bb.intents)
bb.atexit.register(botClient.shutdown)
botClient.run(bb.lib.cfg['discord']['token'])
```
If you need to modify any of the core behaviour of the bot.py, you can inherit it like so and override its' core funcitons:
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