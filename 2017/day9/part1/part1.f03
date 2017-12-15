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

  !call assert_equal(1,  score('{}'))
  !call assert_equal(6,  score('{{{}}}')) ! score 1 + 2 + 3 = 6
  !call assert_equal(5,  score('{{},{}}')) ! score 1 + 2 + 2 = 6
  !call assert_equal(16, score('{{{},{},{{}}}}')) ! score 1 + 2 + 3 + 3 + 3 + 4 = 16
  !call assert_equal(1,  score('{<a>,<a>,<a>,<a>}')) ! score 1
  !call assert_equal(9,  score('{{<ab>},{<ab>},{<ab>},{<ab>}}')) ! score 1 + 2 + 2 + 2 + 2 = 9
  !call assert_equal(9,  score('{{<!!>},{<!!>},{<!!>},{<!!>}}')) ! score 1 + 2 + 2 + 2 + 2 = 9
  !call assert_equal(3,  score('{{<a!>},{<a!>},{<a!>},{<ab>}}')) ! score 1 + 2 = 3
  print *, score(str)
end program hello

