"
A scanner traverses its range in R+R-2 (or 2R-2) time -- R-1 to traverse all but the last position,
and R-1 again to traverse back to all but the first position.

So really, all we need to do is figure out some wait value W such that
(W + Sdi) mod (2Sri-2) != 0 for all i, 
where Sdi is the depth of Scanner i and Sri is the range of scanner r.

Since (W + Sdi) mod (2Sri - 2) will be 0 whenever (W+Sdi) is a multiple of (2Sri-2),
this boils down to finding the smallest W for which (W+Sdi) is relatively prime with (2Sri-2) for all S.

We'll test that we have the detection criteria correct by first verifying
that for W=0 we get the same collisions with this formula that we would have
with the original simulation (i.e., at depths 0 and 6 for the test input).
"

Object subclass: Firewall [
  |scanners|

  Firewall class >> new [
    |f|
    f := super new.
    f init.
    ^f
  ]

  init [
    scanners := Dictionary new
  ]

  putScanner: s at: d [
    scanners at: d put: s
  ]

  getScanners [
    ^scanners
  ]
]

Object subclass: Scanner [
  |range position direction|

  Scanner class >> new: r [
    |s|
    s := Scanner new.
    s init: r.
    ^s
  ]

  init: r [
    range := r.
    position := 1.
    direction := 1.
  ]

  getRange [
    ^range
  ]

  getPosition [
    ^position
  ]

  iterate [
    (position + direction > range) ifTrue: [
      direction := -1.
    ].
    (position + direction < 1) ifTrue: [
      direction := 1.
    ].
    position := position + direction.
  ]
]

Object subclass: Traversal [
  |firewall delay|
  Traversal class >> withFirewall: f withDelay: d [
    |t|
    t := Traversal new.
    t init: f delay: d.
    ^t
  ]

  init: f delay: d [
    firewall := f.
    delay := d.
  ]

  printOn: stream [
    |scanners range|
    scanners := firewall getScanners.

    scanners keys sorted do: [:depth |
      range := (scanners at: depth) getRange.
      stream nextPutAll: 'depth: ';
             nextPutAll: depth asString;
             nextPutAll: '; range: ';
             nextPutAll: range asString;
             nextPutAll: '; time reached: ';
             nextPutAll: (delay + depth) asString;
             nextPutAll: '; scanner position: ';
             nextPutAll: ((delay + depth) \\ (2*range - 2)) asString;
             nl.
    ].
  ]

  displayScanner: scanner at: depth on: stream [
    stream nextPutAll: (depth asString);
           nextPutAll: ': range: ';
           nextPutAll: (scanner getRange asString);
           nextPutAll: ' position: ';
           nextPutAll: (scanner getPosition asString);
           nl.
  ]

  detection [
    |scanners range|
    scanners := firewall getScanners.

    scanners keys sorted do: [:depth |
      range := (scanners at: depth) getRange.
      (((delay + depth) \\ (2*range - 2)) = 0) ifTrue: [
        ^depth
      ]
    ].

    ^false
  ]
]

firewall := Firewall new.

input := FileStream open: '/dev/stdin' mode: FileStream read.
input linesDo: [ :c| 
  tokens := c substrings: ': '.

  depth := (tokens at: 1) asNumber.
  range := (tokens at: 2) asNumber.

  firewall putScanner: (Scanner new: range) at: depth .
].

Transcript nl.

delay := 0.
traversal := Traversal withFirewall: firewall withDelay: delay.
detectionDepth := traversal detection.
[detectionDepth = false] whileFalse: [
  (detectionDepth < 37) ifFalse: [
    Transcript nextPutAll: 'delay ';
               nextPutAll: delay asString;
               nextPutAll: ' detected at ';
               nextPutAll: detectionDepth asString;
               nl.
  ].
  delay := delay + 1.
  traversal := Traversal withFirewall: firewall withDelay: delay.
  detectionDepth := traversal detection.
].

Transcript nextPutAll: 'Undetected at: ';
           nextPutAll: delay asString;
           nl.
