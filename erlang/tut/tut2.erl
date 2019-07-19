-module(tut2).
-export([convert/1]).

convert({in, M}) ->
    M / 2.54;
convert({cm, M}) ->
    M * 2.54.
