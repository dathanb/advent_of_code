module io
  implicit none
contains
  subroutine read_file(filename, str, length)
    ! cribbed from https://software.intel.com/en-us/forums/intel-visual-fortran-compiler-for-windows/topic/542533
    implicit none
    character(len=*),intent(in) :: filename
    character(len=:),allocatable,intent(out) :: str
    integer,intent(out) :: length
    !local variables:
    integer :: iunit,istat
    character(len=1) :: c

    open(newunit=iunit,file=filename,status='OLD',form='UNFORMATTED',access='STREAM',iostat=istat)

    if (istat==0) then
      !how many characters are in the file:
      inquire(file=filename, size=length)
      if (length>0) then

        !read the file all at once:
        allocate( character(len=length) :: str )
        read(iunit,pos=1,iostat=istat) str

        if (istat==0) then
        !make sure it was all read by trying to read more:
        read(iunit,pos=length+1,iostat=istat) c
        if (.not. IS_IOSTAT_END(istat)) &
          write(*,*) 'Error: file was not completely read.'
        else
          write(*,*) 'Error reading file.' 
        end if

        close(iunit, iostat=istat)
      else
        write(*,*) 'Error getting file size.'
      end if
    else
      write(*,*) 'Error opening file.'
    end if

  end subroutine read_file 
end module io
