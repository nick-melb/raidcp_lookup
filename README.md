# Pokemon Go Raid Hundo CP Lookup Tool
CLI lookup tool to enable quick lookup of the CP values for the Hundo of Pokemon in raids


# Status

Should now be stable enough and not crash anymore for invalid Pokemon numbers. If bad CSV data is supplied it will still generally pass compiler checks but cause verbose non-panic errors at runtime when a bad entry is looked up.

## Todo

1. Most Urgent: finish populating the data csv. Currently only has a small dataset of Pokemon that were in rotation in early April for testing purposes.
2. Tidy up print statements for easier reading
3. Would be nice to develop into a Discord bot or similar for ease of access.
4. Implement stdin args to be passed in and stout for pipes (main loop will need rework/alt flow)

## Current Features
1. Pokemon lookup by Pokédex Number or by name (partial matches are accepted)
2. Hundo CP at level 20 and level 25 (weather boost)
3. Weather conditions required for weather boost to be enabled in game
4. Pokemon type(s)
5. Shiny availability in game

## General Notes

To keep the binary portable the CSV data gets compiled in. This may not be desirable in some circumstances.
