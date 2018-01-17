// test input
//const SEED1 = 65;
const SEED1 = 883;
const FACTOR1 = 16807;

// test input
//const SEED2 = 8921;
const SEED2 = 879;
const FACTOR2 = 48271;

const DIVISOR = 2147483647;

const COUNT = 2;

class Generator extends Iterable<num> {
  var _seed;
  var _factor;

  Generator(this._seed, this._factor) {
  }

  num get seed {
    return this._seed;
  }

  Iterator<num> get iterator {
    return new GeneratorIterator(this);
  }

  num calcNext(num previous) {
    var val = previous * this._factor;
    var remainder = val % DIVISOR;
    return remainder;
  }
}

class GeneratorIterator extends Iterable<num> {
  var generator;
  var n;

  GeneratorIterator(this.generator) {
    this.n = this.generator.seed;
  }

  num get current {
    return this.n;
  }

  bool moveNext() {
    this.n = this.generator.calcNext(this.n);
  }
}

class Judge {
  var gen1;
  var gen2;

  Judge(this.gen1, this.gen2) {
  }

  num countMatches(num count) {
    var iterator1 = gen1.iterator;
    var iterator2 = gen2.iterator;

    var matches = 0;

    for (var i=0; i< count; i++) {
      iterator1.moveNext();
      iterator2.moveNext();

      var val1 = iterator1.current;
      var val2 = iterator2.current;

      if ((val1 & 0xffff) == (val2 & 0xffff)) {
        matches += 1;
      }
    }

    return matches;
  }
}

void main() {
  var gen1 = new Generator(SEED1, FACTOR1);
  var gen2 = new Generator(SEED2, FACTOR2);

  var judge = new Judge(gen1, gen2);

  print(judge.countMatches(40000000));
}
