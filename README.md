# Power calculator for Arknights: Endfield

Stop wasting batteries!

This tool calculates the optimal splitter chain (divider stack) that will minimize your battery usage while still keeping the lights on, and your factory running.

(output example)

## What do the outputs mean?

### Build

This is what you're supposed to build
- **full thermal banks**: build this many fully powered Thermal Banks
- **divider stack**: layout of the divider stack 
	- **length**: length of the divider stack
	- **3s**: number of 3-way splits
	- **2s**: number of 2-way splits

(the order of 3-way and 2-way splits doesn't matter)

(build example)

### Stats

Info about the generated setup
- **PAC power reserve**: how long the reserve power will last
	- **power deficit**: self explanatory
	- **lowest reserve**: lowest the reserve power will go
		- During testing I found, that the game doesn't display reserve power below 3% / 3K, instead showing 0. **This is only visual**, your facilities will continue working until power actually drops to 0.
	- **charge time**: time to fully charge after a battery is inserted
	- **efficiency**: how much of the battery's total power is used
	- **power headroom**: how much extra power draw the system can handle, before you need to reconfigure again
- **div stack feed interval**: interval between battery insertions
	- **off time**: idle time of the modulated Thermal Bank
	- **feed rate**: battery consumption of the modulated Thermal Bank
- **total battery consumption**: battery consumption of the whole system
