# Site Update Checker
This is a program that will check for changes to part of a static webpage, and will send a notification using a HTTP POST message, for use with apps such as [ntfy.sh](https://ntfy.sh/).
It is intended to be run regularly, using a task scheduler such as Cron.

This application is most likely to only work with static sites without bot precautions, so please keep that in mind.

## Build
Before compilation, make sure `url.txt` is populated with a URL in order for notifications to be sent.

## Usage
Run the program without flags to check all added targets once.
Id refers to the id or unique class of an element in the document, and must be prefixed with either . or #, depending on type.  
*note that some interfaces require the # symbol to be escaped using a backslash*

### Flags
`-a [URL] [id]` - Add URL and element id to list of targets  
`-d [URL]` - Delete specified URL from list of targets  
`-l` - List all targets  
`-h -help` - Help dialogue