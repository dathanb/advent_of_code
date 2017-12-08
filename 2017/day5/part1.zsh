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
  #echo step: $steps
  rel=$lines[$index]
  #echo move $rel
  (( lines[$index] = $lines[$index] + 1 ))
  (( index += $rel ))
  #echo new index: $index
  [[ $index -lt 1 ||  $index -gt $len ]] && break
done

echo $steps
