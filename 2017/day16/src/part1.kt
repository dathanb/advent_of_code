import java.io.File

class DanceLineup(val lineup: String = "abcdefghijklmnop") {
  fun spin(count: Int): DanceLineup {
    val splitIndex = lineup.length - count;
    return DanceLineup(lineup.substring(splitIndex) + lineup.substring(0, splitIndex))
  }

  fun exchange(first: Int, second: Int): DanceLineup {
    val lesser = if (first < second) first else second
    val greater = if (first < second) second else first
    var ret = lineup.substring(0, lesser)
    ret += lineup.substring(greater, greater+1)
    ret += lineup.substring(lesser+1, greater)
    ret += lineup.substring(lesser, lesser+1)
    ret += lineup.substring(greater + 1)
    return DanceLineup(ret)
  }

  fun partner(first: String, second: String) : DanceLineup {
    return exchange(lineup.indexOf(first), lineup.indexOf(second))
  }

  fun dispatch(operation: String) : DanceLineup {
    print("$operation: ")
      if(operation.startsWith("x")) {
        // exchange
          val (first, second) = operation.substring(1).split("/")
        val newLineup = exchange(first.toInt(), second.toInt())
          println(newLineup)
        return newLineup
      } else if (operation.startsWith("p")) {
        // partner
        val (first, second) = operation.substring(1).split("/")
        val newLineup = partner(first, second)
        println(newLineup)
        return newLineup
      } else if (operation.startsWith("s")) {
        // spin
          val len = operation.substring(1)
        val newLineup = spin(len.toInt())
        println(newLineup)
        return newLineup
      } else {
        throw RuntimeException("Unrecognized move: $operation")
      }
  }

  override fun toString() : String {
    return lineup
  }
}

class DanceCaller(val input: String, val lineup: DanceLineup) {
  fun dance(): DanceLineup {
    return input.split(",").fold(lineup, {acc, operation -> acc.dispatch(operation)})
  }
}

fun main(args: Array<String>) {
  var lineup = DanceLineup("abcdefghijklmnop")
  val input = File("input").readText()

  DanceCaller(input, lineup).dance()
}

