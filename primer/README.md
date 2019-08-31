# Primer markup
Pseudo markup format for Robo Instructus containing the programming primer
received through the game's story. This is a custom markup for the game's uses.

Markup format:
* `#!unlock` means the start of a section, the lines following
are the message in simplified markdown.
* Most of the time the first line, starting with `# ` is the subject.
* Newlines always count, unlike markdown.
* Backticks wrap inline bits of code, e.g <code>\`var foo = q\`</code>.
* Three backticks <code>\`\`\`</code> wrap around code blocks. They may have text immediately
after them, e.g.<code>\`\`\`no_run</code> which isn't displayed & should be left in english.
In code blocks only comments should be translated.

# Example
~~~text
#!unlock type{primer} id{loops}
# Loops

`loop` is used to repeat commands. It will repeat the commands in its scope forever. A loop's scope is the commands indented that come after a `loop` line.

```max_run=11
robo_forward()  # called once

loop
    robo_forward()
    robo_left()

robo_left()  # never called
```
~~~

![](https://user-images.githubusercontent.com/2331607/64056164-39ab5a80-cb89-11e9-9ebd-6e7d976d4974.jpg)
