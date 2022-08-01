# simple-regex

This is a simple regular expression parsing library, adapted from this [digest](https://www.cs.princeton.edu/courses/archive/spr09/cos333/beautiful.html),
that supports the following pattern elements:

* `c` : any character not listed below; matches itself
* `.` : wildcard; matches any character
* `^` : begin; matches at the beginning of the text
* `$` : end; matches at the end of the text
* `*` : repeat: matches previous character zero or more times

The package also contains a minimal executable to demonstrate its functionality. The following is an example of its usage:

```
$ simple-regex ^abc abcdefg
The input text [ abcdefg ] does match the pattern [ ^abc ]
```
