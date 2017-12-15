module test
  implicit none
contains
  subroutine assert_equal(expected, actual)
    implicit none
    integer,intent(in) :: expected
    integer,intent(in) :: actual

    if (expected /= actual) then
      print *, "Expected ",expected," but got ",actual
    end if
  end subroutine assert_equal
end module test
