## Initialisation
import boilerBot.lib as lib
import nextcord
from typing import Optional
from nextcord.ext import commands, tasks

## Define help cog
class help(commands.Cog):
    ## Initialose with help info
    def __init__(self,bot):
        self.bot = bot
        self.category = lib.getCategory(self.__module__)
        self.description = f"Display help about {self.bot.user.name}'s commands"
        self.usage = f"""
        {self.bot.command_prefix}help
        {self.bot.command_prefix}help <command>
        """
        self.hidden = False
        
    ## Callable command to provide user help with command usage
    @commands.command(aliases=["?"],name="help")
    async def prefhelp(self,ctx,*args):
        if not args:
            args = [None]
        return await self.help(ctx,args[0])

    @nextcord.slash_command(name="help")
    async def slashhelp(self,ctx,command:Optional[str]):
        """Get help on how to use this bot.

        Parameters
         ----------
        ctx: Interaction
            The interaction object
        command: str
            Optional: Specify a command for more detailled usage info
        """
        return await self.help(ctx,command)

    async def help(self, ctx, arg):
        embed=False
        ## Provide specific help, or general command list
        if (arg) :
            cog = self.bot.get_cog(arg)
            command = self.bot.get_command(arg)
            if not (cog):
                pass
            ## Gather usage info about command
            elif (not cog.hidden):
                embed=lib.embed(
                    title=cog.qualified_name,
                    description=cog.description,
                    sections=[("Usage",cog.usage),("Category",cog.category)]
                )
                if (command.aliases):
                    embed.set_footer(text=f'Aliases: {", ".join(command.aliases)}')
        else:
            cogs = {}
            for cog in self.bot.cogs:
                cog = self.bot.get_cog(cog)
                if (not cog.hidden):
                    if not (cog.category in cogs.keys()):
                        cogs[cog.category] = []
                    cogs[cog.category].append(f"`{cog.qualified_name}`\n> {cog.description}")
            ## Display list of commands and descriptions
            embed=lib.embed(
                title="List of commands:",
                footer=f"Use {self.bot.command_prefix}help <command> to get more specific usage information."
            )
            for category in cogs.keys():
                embed.add_field(name=category,value="\n".join(cogs[category]))
                
        if not (embed):   
            embed=lib.embed(
                title="This command does not exist",
                description=f"Try {self.bot.command_prefix}help to see a list of available commands."
            )       
        await ctx.send(embed=embed)



def setup(bot):
    bot.add_cog(help(bot))
