from boilerBot.lib import cfg

## Does await stuff with messages, for cleaning
async def send(ctx, embed, previous=None):
    if previous and cfg["options"]["embed"]["cleanup"]:
        await previous.delete()
        return await ctx.send(embed=embed)
    else:
        return await ctx.send(embed=embed)

## Delete the message passed to the function, intended to be the bot's last message
async def clean(previous=None):
    if previous:
        await previous.delete()
    return None