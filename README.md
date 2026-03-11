# edc
A very small and simple program for decapitalising file extensions.

```
De-capitalises all uppercase or mixed-case file extensions in the current
working directory (by default).

Usage: edc [OPTIONS] [PATH]...

Arguments:
  [PATH]...  Where to look for uppercase extensions

Options:
  -r, --recursive    Recursively check through sub-directories
  -v, --verbose      Print files as they have their extensions de-capitalised
  -q, --quiet        Don't produce any output (extensions won't be de-capitalised 
                     if it'll cause cause a pre-existing file to be overwritten)
  -i, --interactive  Prompt before fixing extension
  -h, --help         Print help
  -V, --version      Print version
```
