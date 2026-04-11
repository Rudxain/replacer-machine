# replacer machine
Rewrite/replacement automaton that works like this:

The program is an ordered list of rules. Each rule maps a constant substring (the "pattern") to a constant literal. The automaton tries matching each rule in order:
- if a rule matches, all non-overlapping occurrences of pattern in the tape are replaced by the associated literal, then the tape is printed to `stdout`, then the rule-pointer is reset to `0` (`goto` first rule)
- if a rule doesn't match, the automaton tries the next rule
- if no more rules match (rule-pointer is out-of-bounds), the automaton halts

Notice that I said *"match"* not *"change"*. A rule can match even if it doesn't change the tape.

I'm working on allowing the program to specify which replacements to print, and which occurrences to replace. `<` to replace-first, `>` to replace-last, `@` to replace-all.

## Implications
Notice that *duplicate patterns can change the behavior* of a machine, so they are not "keys" in a map. However, if 2 or more consecutive rules match the same pattern, only the 1st one in that chain can have an effect, all further dupes in the chain are essentially unreachable.

## Name
Please suggest names, this name is so bad 💀
