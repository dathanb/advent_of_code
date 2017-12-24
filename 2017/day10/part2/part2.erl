-module(part2).
-export([solution/0]).

solution() ->
  list_to_hex(dense_hash(sparse_hash(build_list(255), augmented_input(input())))).

input() ->  % in Erlang strings are just lists of bytes, so we won't have to do anything special to handle this as a list of ascii characters
  "31,2,85,1,80,109,35,63,98,255,0,13,105,254,128,33".

augmented_input(Input) ->
  Input ++ [17, 31, 73, 47, 23].

list_to_hex(Nums) ->
  lists:flatten(
    lists:map(
      fun(X)->io_lib:format("~2.16.0B", [X]) end, 
      Nums)).

dense_hash(Nums) ->
  [
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(Nums,16)),
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(drop(Nums,16), 16)),
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(drop(Nums,2*16), 16)),
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(drop(Nums,3*16), 16)),
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(drop(Nums,4*16), 16)),
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(drop(Nums,5*16), 16)),
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(drop(Nums,6*16), 16)),
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(drop(Nums,7*16), 16)),
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(drop(Nums,8*16), 16)),
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(drop(Nums,9*16), 16)),
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(drop(Nums,10*16), 16)),
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(drop(Nums,11*16), 16)),
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(drop(Nums,12*16), 16)),
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(drop(Nums,13*16), 16)),
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(drop(Nums,14*16), 16)),
    lists:foldl(fun(E,A)-> E bxor A end, 0, take(drop(Nums,15*16), 16))
  ].

sparse_hash(Nums, Inputs) ->
  nth_knot(Nums, Inputs, 64).

nth_knot(Nums, Lengths, N) ->
  Knot = nth_knot(Nums, Lengths, 0, 0, N).
nth_knot(Nums, _, _, _, 0) ->
  Nums;
nth_knot(Nums, Lengths, Skip, Pointer, N) ->
  {NewNums, NewSkip, NewPointer} = knot(Nums, Lengths, Skip, Pointer),
  nth_knot(NewNums, Lengths, NewSkip, NewPointer, N-1).

hash(Nums, Lengths) ->
  NewList = knot(Nums, Lengths, 0, 0),
  lists:nth(1, NewList) * lists:nth(2, NewList).

knot(Nums, [], Skip, Pointer) ->
  {Nums, Skip, Pointer};
knot(Nums, Lengths, Skip, Pointer) ->
  [Head|Tail] = Lengths,
  NewNums = reverse_circular_slice(Nums, Pointer, Head),
  NewSkip = Skip + 1,
  NewPointer = ((Pointer + Head + Skip) rem list_length(Nums)),
  knot(NewNums, Tail, NewSkip, NewPointer).

build_list(N) ->
  cons_list(0, N).

cons_list(N,Max) when N == Max ->
  [Max];
cons_list(N, Max) ->
  [N|cons_list(N+1, Max)].

reverse_circular_slice(L, _, 0) ->
  L;
reverse_circular_slice(L, _, 1) ->
  L;
reverse_circular_slice(L, Start, Length) ->
  NewList = swap(L, Start, (Start + Length - 1) rem list_length(L)),
  NewStart = (Start+1) rem list_length(L),
  reverse_circular_slice(NewList, NewStart, Length - 2).

swap(L, Left, Right) when Left > Right ->
  swap(L, Right, Left);
swap(L, Left, Right) ->
  LeftItem = lists:nth(Left+1, L),
  RightItem = lists:nth(Right+1, L),
  take(L, Left) ++ [RightItem] ++ take(drop(L, Left+1), (Right-Left-1)) ++ [LeftItem] ++ drop(L, Right+1).

drop([], N) ->
  [];
drop(L, 0) ->
  L;
drop([Head | Tail], N) ->
  drop(Tail,N-1).

take([], N) ->
  [];
take(List, 0) ->
  [];
take([Head | Tail], N) ->
  [Head | take(Tail, N-1)].

list_length(L) ->
  list_length(L, 0).

list_length([], Count) ->
    Count;
list_length([_ | Tail], Count) ->
  list_length(Tail, Count+1).

