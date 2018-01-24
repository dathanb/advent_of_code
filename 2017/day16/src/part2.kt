import java.io.File
import java.util.HashMap

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
      if(operation.startsWith("x")) {
        // exchange
          val (first, second) = operation.substring(1).split("/")
        val newLineup = exchange(first.toInt(), second.toInt())
        return newLineup
      } else if (operation.startsWith("p")) {
        // partner
        val (first, second) = operation.substring(1).split("/")
        val newLineup = partner(first, second)
        return newLineup
      } else if (operation.startsWith("s")) {
        // spin
          val len = operation.substring(1)
        val newLineup = spin(len.toInt())
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

  val lineupsSoFar = HashMap<String, Int>()

  var iteration = 0

  while (!lineupsSoFar.containsKey(lineup.lineup)) {
    lineupsSoFar.put(lineup.lineup, iteration)
    iteration += 1
    lineup = DanceCaller(input, lineup).dance()
      println("$iteration: ${lineup.lineup}")
  }

  println("Found loop at iteration $iteration")
  // In our case it loops at iteration 30. 1 billion modulo 30 is 10, so the answer is
  // whatever lineup we see at iteration 10
}

