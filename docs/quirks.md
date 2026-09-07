# Quirks

This is a place for me to make note of all drive quirks I should be aware of when developing the application side of ChromaDisc.

A quirk is any way in which a drive diverges from the specfications that define it.

## Known Quirks (By Drive)

### ASUS DRW-24B1ST j

- C2 Error Code 01b for the READ CD and READ CD MSF commands results in 296 bytes of C2 data as opposed to the expected 294, suggesting the drive treats code 01b the same as code 10b, which also (correctly) results in 296 bytes. See this [CUETools Issue](https://github.com/gchudov/cuetools.net/issues/107).
