-module(part1).
-export([build_list/1,reverse/3,swap/3,take/2,drop/2,knot/4,hash/2]).

hash(Nums, Lengths) ->
  NewList = knot(Nums, Lengths, 0, 0),
  lists:nth(1, NewList) * lists:nth(2, NewList).

knot(Nums, [], _, _) ->
  Nums;
knot(Nums, Lengths, Skip, Pointer) ->
  [Head|Tail] = Lengths,
  NewNums = reverse(Nums, Pointer, Head),
  NewSkip = Skip + 1,
  NewPointer = ((Pointer + Head + Skip) rem list_length(Nums)),
  knot(NewNums, Tail, NewSkip, NewPointer).

build_list(N) ->
  cons_list(0, N).

cons_list(N,Max) when N == Max ->
  [Max];
cons_list(N, Max) ->
  [N|cons_list(N+1, Max)].

reverse(L, _, 0) ->
  L;
reverse(L, _, 1) ->
  L;
reverse(L, Start, Length) ->
  NewList = swap(L, Start, (Start + Length - 1) rem list_length(L)),
  NewStart = (Start+1) rem list_length(L),
  reverse(NewList, NewStart, Length - 2).

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

