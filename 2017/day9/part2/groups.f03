module groups
contains
  integer function score(input)
    character(len=*),intent(in) :: input
    integer :: i
    integer :: the_score

    the_score = 0
    i = 1

    print *, "scoring input: ", input
    if (input(i:i) == "{") then
      call consume_group(input, 1, i, 0, the_score)
    else if (input(i:i) == "<") then
      call consume_garbage(input, 1, i, the_score)
    end if

    score = the_score
  end function score

  recursive subroutine consume_group(input, i, new_index, depth, score)
    character(len=*),intent(in) :: input
    integer,intent(in) :: i
    integer,intent(in) :: depth

    integer,intent(out) :: score
    integer,intent(out) :: new_index
    integer :: new_depth
    character :: ch

    ! assume input(i:i) is '{'
    new_index = i
    new_depth = depth + 1

    do
      new_index = new_index + 1

      if (new_index > len(input)) then
        exit
      end if

      ch = input(new_index:new_index)

      if (ch == '{') then
        call consume_group(input, new_index, new_index, new_depth, score)
      else if (ch == '}') then
        ! return from this call up to the caller
        exit
      else if (ch == '<') then
        call consume_garbage(input, new_index, new_index, score)
      else if (ch == '!') then
        ! skip the next character, no matter what it is
        new_index = new_index + 1
      end if
    end do
  end subroutine consume_group

  subroutine consume_garbage(input, i, new_index, score)
    character(len=*),intent(in) :: input
    integer,intent(in) :: i
    integer,intent(out) :: new_index
    integer,intent(out) :: score
    character :: ch

    ! i points to the < character
    do
      new_index = new_index + 1
      if (new_index > len(input)) then
        exit
      end if
      ch = input(new_index:new_index)

      if (ch == '!') then
        ! just skip the character
        new_index = new_index + 1
      else if (ch == '>') then
        exit
      else
        score = score + 1
      end if
    end do
  end subroutine consume_garbage
end module groups
