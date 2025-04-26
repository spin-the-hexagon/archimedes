# Content Scripts

Content Scripts are the next-generation of all data in Project ADHD.

> defining *content*
> 
> content is:
>   - saves
>   - beatmaps
>   - any in game data, such as mandated beatmaps, or purchasable beatmaps

Right now, Content is stored in different ways, Content Scripts
aims to stop that.

## Inspiration

Content Scripts contains elements of `V2` and `RAAE`.

More specifically, it takes the powerful opcode system of `V2`,
with the readability and list-encoding of `RAAE`.

## Specification

A content script's core data structure is the instruction list,
which is an encoded list.

This list encoding format differs slightly from `RAAE`, as it's
designed for efficiency.

The list encoding format is a big string, each list item optionally
starts with a Container Change indicator, then with required content and end container.

Examples:

- `ABCDEFG[` (uses the same container as the previous item, so no need to explicitly write it out)
- `$<ABCDEFG<` (uses a different container than the previous item, so we start with a dollar sign and then the container
  character)

Once you have decoded the list, you get to the meat of the script, the instructions.

An example instruction list might look like the following.

- `0` we're calling opcode id 0, which is always Reverse Opcode Id
- `write_log` we're specifying we want to register opcode `write_log`, so now we have `write_log` as opcode 1
- `1` call write_log
- `hello, world` we're writing hello world
- `1` call again
- `i like the world` we're writing i like the world

This script encoded using list encoding would look like the following:

```adhd
$i0i$[write_log[1[hello, world[1[i like the world[
```

The beautiful thing about content scripts is code simplicity, just have one sprite manage all Content Scripts,
with a global list called Content.ScriptQueue, with a queue of scripts to run.

To load a save, you add the save to Content.ScriptQueue.
To load all the games beatmaps into a table, add the table to Content.ScriptQueue.
To load one beatmap, add the beatmap to Content.ScriptQueue.

The one silly part about this is that a beatmap could theoretically mess with save data.

There is one last, super-simple part to the Content Script specification, which is that to make sure a user
pasted in a Content Script, all Content Scripts start with `V3`