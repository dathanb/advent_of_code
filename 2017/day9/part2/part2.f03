!{}, score of 1.
!{{{}}}, score of 1 + 2 + 3 = 6.
!{{},{}}, score of 1 + 2 + 2 = 5.
!{{{},{},{{}}}}, score of 1 + 2 + 3 + 3 + 3 + 4 = 16.
!{<a>,<a>,<a>,<a>}, score of 1.
!{{<ab>},{<ab>},{<ab>},{<ab>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
!{{<!!>},{<!!>},{<!!>},{<!!>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
!{{<a!>},{<a!>},{<a!>},{<ab>}}, score of 1 + 2 = 3.

program hello
  use io
  use test
  use groups
  implicit none
  character,external :: read_char
  character :: ch
  character(len=:),allocatable :: str
  integer :: depth
  integer :: data_length

  CALL read_file('input', str, data_length)

  !call assert_equal(0,  score('<>'))
  !call assert_equal(17, score('<random characters>'))
  !call assert_equal(3,  score('<<<<>'))
  !call assert_equal(2,  score('<{!>}>'))
  !call assert_equal(0,  score('<!!>'))
  !call assert_equal(0,  score('<!!!>>'))
  !call assert_equal(10, score('<{o"i!a,<{i<a>'))
  print *, score(str)
end program hello

