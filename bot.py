## Initialisation
import os
import re
import json
import atexit
import discord
from discord.ext import commands
from . import lib
from . import corecommands
## Constants and Config
intents = discord.Intents.default()
intents.members = lib.cfg['discord']['intents']['members']
intents.guilds = lib.cfg['discord']['intents']['guilds']
intents.reactions = lib.cfg['discord']['intents']['reactions']

## Define bot class
class bot(commands.Bot):
    def __init__(self, *args, **kwargs):
        super().__init__(*args,**kwargs)

        ## Remove default help command to replace with custom one
        self.remove_command('help')

        ## Load corecommands
        for root, subdirs, files in os.walk(os.path.join(os.path.dirname(__file__),"corecommands")):
            if not ("__pycache__" in root):
                for file in files:
                    if file.endswith(".py") and not (file == "help.py" or file == "about.py"):
                        name = file[:-3]
                        self.load_extension(f"boilerBot.corecommands.{name}")

        ## Load plugins
        for root, subdirs, files in os.walk('./commands'):
            if not ("__pycache__" in root):
                for file in files:
                    if file.endswith(".py"):
                        name = file[:-3]
                        parent = ".".join(re.findall(r"\w+",root))
                        self.load_extension(f"{parent}.{name}")
        
        ## Load per-guild variables
        try:
            with open("guildVars.json","r") as file:
                self.guildVars = json.load(file)
        except FileNotFoundError:
            self.guildVars = {}
        try:
            self.guildVarTemplate = kwargs["defaultGuildVars"] 
        except KeyError:
            self.guildVarTemplate = {}

    ## Post login activity
    async def on_ready(self):
        ## Load commands that require login to init
        self.load_extension("boilerBot.corecommands.help")
        self.load_extension("boilerBot.corecommands.about")
        ## Log ready
        lib.log('--------------------------------')
        lib.log('Bot Logged into the APi and initialised.')
        lib.log(f'Logged in as {self.user.name}')
        lib.log(f'User ID: {self.user.id}')
        lib.log('--------------------------------')

    ## Setup guildVars for new guild
    async def on_guild_join(self, guild):
        self.guildVars[str(guild.id)] = self.guildVarTemplate
    
    ## Dump guildVars on guild remove
    async def on_guild_remove(self,guild):
        del self.guildVars[str(guild.id)]
    
    ## Inform user of error
    async def on_command_error(self, ctx, err):
        embed=lib.embed(
            title="Error running command:",
            description=err.args[0],
            colour=lib.errorColour
        )
        await ctx.send(embed=embed)

    ## Store current state of guildVars in a json on shutdown to persist over restarts
    def shutdown(self):
        with open("guildVars.json","w+") as file:
            json.dump(self.guildVars,file)

#Testing, will not run when imported
if __name__ == "__main__":
    ## Create an instance of the bot. You'll need this or similar in your main file
    botClient = bot(lib.cfg['options']['prefix'], intents=intents)
    atexit.register(botClient.shutdown)
    botClient.run(lib.cfg['discord']['token'])
