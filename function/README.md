# Function doc markup
Pseudo markup format for Robo Instructus containing documentation for the
functions unlocked during the game. This is a custom markup for the game's uses.

Markup format:
* `#!unlock` means the start of a section, the lines following are the message in simplified markdown.
* Most of the time the first line, starting with `# ` is the subject.
* Newlines always count, unlike markdown.
* Backticks wrap inline bits of code, e.g <code>\`var foo = q\`</code>. Inline code shouldn't be translated.
* Three backticks <code>\`\`\`</code> wrap around code blocks. They may have text immediately
after them, e.g.<code>\`\`\`no_run</code> which isn't displayed & should be left in english.
In code blocks only comments should be translated.
* Text like `$tu{robo_forward()}` will be replaced in with the runtime cost of the function ie `450`. These shouldn't be modified.
* Text like `$render{robo_forward}` will indicate where to render the function
in action "image" in-game. These shouldn't be modified.
* `type{function(s)}` new function(s) unlock
* `type{function-update}` new docs for a function, like a new scan code. Replaces previous docs.
* `type{short}` short version function doc that appears top-left above the code input. Note: The extra space after the `)` in <code>\`robo_use() \` use curr...</code> is intentional.

# Example
~~~text
#!unlock type{functions} id{left-forward}
`robo_left()` Each call instructs the robot to move to the next side of the current tile, counter-clockwise. Runtime $tu{robo_left()} Mimas-seconds (ms).
$render{robo_left}
`robo_forward()` Instructs the robot to move forward onto a tile it's currently facing. The robot will favour veering to the right hand side when doing this. Runtime $tu{robo_forward()} ms.
$render{robo_forward}
~~~

![](https://user-images.githubusercontent.com/2331607/64056308-31075400-cb8a-11e9-92e0-e9906d5907f0.jpg)
