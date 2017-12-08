#!/bin/zsh

lines=($(<day5.input))
#lines=($(<test.input))
len=$#lines
#echo $lines
#echo $len


index=1
steps=0

while true; do
  (( steps++ ))
  rel=$lines[$index]
  if [[ lines[$index] -ge 3 ]]; then
    (( lines[$index] = $lines[$index] - 1 ))
  else
    (( lines[$index] = $lines[$index] + 1 ))
  fi
  (( index += $rel ))
  [[ $index -lt 1 ||  $index -gt $len ]] && break
  echo $steps: $index
done

echo $steps
