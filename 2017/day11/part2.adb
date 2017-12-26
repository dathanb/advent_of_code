with Ada.Text_IO;

procedure Part2 is
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
      elsif target.x > currentPosition.x and target.y < currentPosition.y then
        currentPosition := Move(currentPosition, "se");
      elsif target.x < currentPosition.x and target.y > currentPosition.y then
        currentPosition := Move(currentPosition, "nw");
      elsif target.x < currentPosition.x and target.y < currentPosition.y then
        currentPosition := Move(currentPosition, "sw");
      elsif target.x > currentPosition.x then
        currentPosition := Move(currentPosition, "ne");
      elsif target.x < currentPosition.x then
        currentPosition := Move(currentPosition, "nw");
      elsif target.y > currentPosition.x then
        currentPosition := Move(currentPosition, "n" & Character'Val(0));
      else
        currentPosition := Move(currentPosition, "s" & Character'Val(0));
      end if;

      length := length + 1;
    end loop;

    return length;
  end FindPath;

  position : Point;
  direction : String(1..2);
  avail : Boolean := true;
  newPoint : Point;
  greatestDistance: Integer := 0;
  currentDistance: Integer;
begin
  position.x := 0;
  position.y := 0;

  while avail
  loop
    ReadDirection(direction, avail);
    newPoint := Move(position, direction);
    position := newPoint;
    currentDistance := FindPath(position);
    if currentDistance > greatestDistance then
      greatestDistance := currentDistance;
    end if;

  end loop;

  IO.Put("Greatest distance was ");
  Integer_IO.Put(greatestDistance);
  IO.Put_Line("");
end Part2;
