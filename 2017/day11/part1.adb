with Ada.Text_IO;

-- Given some path where the wanderer ends up at a particular coordinate,
-- we need to find the shortest path from origin to that coordinate.
-- I originally thought of implementing Dijkstra's algo, but it seems
-- like I could just define a function that would give you the
-- next step on the shortest path to the end point, and then call that
-- function recursively or iteratively until you've reached the target point.
--
procedure Part1 is
  package IO renames Ada.Text_IO;
package Integer_IO is new Ada.Text_IO.Integer_IO (Integer);

  type Point is
    record
      x : Integer;
      y : Integer;
    end record;

  function Move(start : in Point; direction : in String) return Point is
    newPoint : Point;
  begin
    newPoint.x := 0;
    newPoint.y := 0;
    if (direction(direction'First) = 'n' and direction(direction'First+1) = Character'Val(0)) then
      newPoint.x := start.x;
      newPoint.y := start.y + 2;
    elsif (direction = "ne") then
      newPoint.x := start.x + 1;
      newPoint.y := start.y + 1;
    elsif (direction = "se") then
      newPoint.x := start.x + 1;
      newPoint.y := start.y - 1;
    elsif (direction(direction'First) = 's' and direction(direction'First+1) = Character'Val(0)) then
      newPoint.x := start.x;
      newPoint.y := start.y - 2;
    elsif (direction = "sw") then
      newPoint.x := start.x - 1;
      newPoint.y := start.y - 1;
    elsif (direction = "nw") then
      newPoint.x := start.x - 1;
      newPoint.y := start.y + 1;
    end if;
    return newPoint;
  end Move;

  procedure ReadDirection(item : out String; available : out Boolean) is
    ch : Character;
    tmp : String(1..1);
  begin
    item(item'First) := Character'Val(0);
    item(item'First + 1) := Character'Val(0);
    IO.Get_Immediate(ch, available);
    tmp(tmp'First) := ch;
    if available then
      item(item'First) := ch;
    end if;
    IO.Get_Immediate(ch, available);
    if (available and ch /= ',') then
      item(item'First + 1) := ch;
    end if;
    if (ch = ',') then
      return;
    end if;
    -- If we got here, it means we read two characters and didn't hit a comma or end of input.
    -- Let's consume the next character to make sure that the next call to ReadDirection
    -- will read a direction, not a comma
    begin
      IO.Get_Immediate(ch, available);
    exception
      when IO.End_Error =>
        available := false;
    end;
  end ReadDirection;

  function FindPath(target: Point) return Integer is
    currentPosition: Point;
    length: Integer := 0;
  begin
    currentPosition.x := 0;
    currentPosition.y := 0;

    while target.x /= currentPosition.x or target.y /= currentPosition.y
    loop
      if target.x > currentPosition.x and target.y > currentPosition.y then
        currentPosition := Move(currentPosition, "ne");
        IO.Put("Moving ne to ");
      elsif target.x > currentPosition.x and target.y < currentPosition.y then
        currentPosition := Move(currentPosition, "se");
        IO.Put("Moving se to ");
      elsif target.x < currentPosition.x and target.y > currentPosition.y then
        currentPosition := Move(currentPosition, "nw");
        IO.Put("Moving nw to ");
      elsif target.x < currentPosition.x and target.y < currentPosition.y then
        currentPosition := Move(currentPosition, "sw");
        IO.Put("Moving sw to ");
      elsif target.x > currentPosition.x then
        currentPosition := Move(currentPosition, "ne");
        IO.Put("Moving ne to ");
      elsif target.x < currentPosition.x then
        currentPosition := Move(currentPosition, "nw");
        IO.Put("Moving nw to ");
      elsif target.y > currentPosition.x then
        currentPosition := Move(currentPosition, "n");
        IO.Put("Moving n to ");
      else
        currentPosition := Move(currentPosition, "s");
        IO.Put("Moving s to ");
      end if;

      Integer_IO.Put(currentPosition.x);
      Integer_IO.Put(currentPosition.y);
      IO.Put_Line("");

      length := length + 1;
    end loop;

    return length;
  end FindPath;

  position : Point;
  direction : String(1..2);
  avail : Boolean := true;
  newPoint : Point;
begin
  position.x := 0;
  position.y := 0;

  while avail
  loop
    ReadDirection(direction, avail);
    IO.Put("Next direction: ");
    IO.Put_Line(direction);
    newPoint := Move(position, direction);
    position := newPoint;
  end loop;

  IO.Put_Line("New position is");
  Integer_IO.Put(position.x);
  Integer_IO.Put(position.y);

  IO.Put_Line("Shortest path is ");
  Integer_IO.Put(FindPath(position));
end Part1;
