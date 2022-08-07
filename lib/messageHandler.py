from boilerBot.lib import cfg

## Does await stuff with messages, for cleaning
async def send(ctx, embed, previous=None):
    if previous and cfg["options"]["embed"]["cleanup"]:
        try:
            await previous.delete()
        except:
            pass
        return await ctx.send(embed=embed)
    else:
        return await ctx.send(embed=embed)

## Delete the message passed to the function, intended to be the bot's last message
async def clean(previous=None):
    if previous:
        try:
            await previous.delete()
        except:
            pass
    return None