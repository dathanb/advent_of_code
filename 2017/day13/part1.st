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
  |firewall packetPosition severity|
  Traversal class >> new: f [
    |t|
    t := Traversal new.
    t init: f.
    ^t
  ]

  init: f [
    firewall := f.
    packetPosition := -1.
    severity := 0.
  ]

  printOn: stream [
    |scanners packetDisplayed|
    packetDisplayed := false.
    scanners := firewall getScanners.

    stream nextPutAll: 'Severity: ';
           nextPutAll: severity asString;
           nl.

    scanners keys sorted do: [:depth |
      ((packetDisplayed not) and: [depth >= packetPosition]) ifTrue: [
        packetDisplayed := true.
        self displayPacket: stream.
      ].
      self displayScanner: (scanners at: depth) at: depth on: stream.
    ].
  ]

  displayPacket: stream [
    stream nextPutAll: packetPosition asString;
           nextPutAll: ': *';
           nl.
  ]

  displayScanner: scanner at: depth on: stream [
    stream nextPutAll: (depth asString);
           nextPutAll: ': range: ';
           nextPutAll: (scanner getRange asString);
           nextPutAll: ' position: ';
           nextPutAll: (scanner getPosition asString);
           nl.
  ]

  iterate [
    |scanner|
    packetPosition := packetPosition + 1.
    (firewall getScanners includesKey: packetPosition) ifTrue: [
      scanner := firewall getScanners at: packetPosition.
      (scanner getPosition = 1) ifTrue: [
        severity := severity + (packetPosition * (scanner getRange)).
      ].
    ].

    firewall getScanners keysAndValuesDo: [ :key :scanner|
      scanner iterate.
    ].
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

t := Traversal new: firewall.
Transcript nl.

85 timesRepeat: [
  t iterate.
].

t printOn: stdout.
Transcript nl.
