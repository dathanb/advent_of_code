module groups
contains
  integer function score(input)
    character(len=*),intent(in) :: input
    integer :: i
    integer :: the_score

    the_score = 0
    i = 1

    !print *, "scoring input: ", input
    call consume_group(input, 1, i, 0, the_score)

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

    !print *, "Entering consume_group"
    !print *, "index: ", i
    !print *, "data size: ", len(input)
    !print *, "depth: ", depth
    !print *, "score: ", score

    ! assume input(i:i) is '{'
    new_index = i
    new_depth = depth + 1
    score = score + new_depth

    do
      new_index = new_index + 1

      if (new_index > len(input)) then
        exit
      end if

      ch = input(new_index:new_index)

      !print *, "New index: ", new_index
      !print *, "New character: ", input(new_index:new_index)
      if (ch == '{') then
        call consume_group(input, new_index, new_index, new_depth, score)
      else if (ch == '}') then
        ! return from this call up to the caller
        exit
      else if (ch == '<') then
        call consume_garbage(input, new_index, new_index)
      else if (ch == '!') then
        ! skip the next character, no matter what it is
        new_index = new_index + 1
      end if
    end do
  end subroutine consume_group

  subroutine consume_garbage(input, i, new_index)
    character(len=*),intent(in) :: input
    integer,intent(in) :: i
    integer,intent(out) :: new_index
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
      end if
    end do
  end subroutine consume_garbage
end module groups
