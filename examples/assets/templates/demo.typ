#set page(width: 10cm, height: auto)
#set text(font: "Roboto") 

= Single Binary
This document was made with a binary executable, and nothing else! \

#include "hello.typ"

#v(1em)
#let y1 = 2017
#let y2 = sys.inputs.year_drawn
#let age = y2 - y1
My daughter was #age years old when she drew this:

== Embedded Image
#image("cute.png")
