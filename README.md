i was annoyed with find and locate and wanted an interactive way of searching for files in the command line.
the selected item is copied to the clipboard.

SHOULD work on linux, macos and windows. has not been tested. if you test it for windows, please inform me.

----

planned "features": 
 - wildcard behaviour needs to be corrected to be consistent with unix wildcards (and maybe made platform specific)
 - possibly performance improvements (especially when traversing the file system)
 - fix the fucking weird directory traversel issue on macos (~/Documents and ~/Downloads only gets traversed if that is the base given via argv... and i don't fucking know why)