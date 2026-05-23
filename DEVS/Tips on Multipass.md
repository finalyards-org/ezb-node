# Tips on using Multipass

Tips in addition to the ones mentioned in the repo's actual contents.

## `~/.espressif` on the host (macOS)

The author has noticed that:

- it's good for IDE to have `~/.espressif` populated also on the host

Some suggestion:

1. Build some subproject within the IDE console

	- open terminal
	- `$ cargo build --release -vv` or the like

	This builds the `~/.espressif` folder, and takes around 5.4GB.
	
	What we have now are e.g. the headers that 
	
	// tbd. did we tho?