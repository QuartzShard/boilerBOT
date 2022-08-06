
# Check an entry for guild in guildvars exists, if not, create one
def retrieve(guildid, bot):
    try:
        guildVars = bot.guildVars[guildid]
    except KeyError:
        guildVars = bot.guildVarTemplate
        bot.guildVars[guildid] = guildVars
    return guildVars

# Set values in guild storage
def set(guildid, bot, guildvars):
        bot.guildVars[guildid] = guildvars
