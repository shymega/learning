#!/usr/bin/env -S swipl -q -t main

hello_world :-
    write("Hello, world!"),
    nl.

main :-
    hello_world(),
    halt.
