import scala.collection.JavaConverters._
import java.util.Scanner

object HelloWorld {
  var comparisonOperators: Map[String, (Int, Int) => Boolean] = Map()
  var mutationOperators: Map[String, (Int, Int) => Int] = Map()

  def main(args: Array[String]) {
    val scanner = new java.util.Scanner(System.in)//.useDelimiter("\\w+");

    val eqOperator = (registerValue: Int, value: Int) => (registerValue == value)
    val neqOperator = (registerValue: Int, value: Int) => (registerValue != value)
    val gtOperator = (registerValue: Int, value: Int) => (registerValue > value)
    val gteOperator = (registerValue: Int, value: Int) => (registerValue >= value)
    val ltOperator = (registerValue: Int, value: Int) => (registerValue < value)
    val lteOperator = (registerValue: Int, value: Int) => (registerValue <= value)
    comparisonOperators = comparisonOperators + ("==" -> eqOperator)
    comparisonOperators = comparisonOperators + ("!=" -> neqOperator)
    comparisonOperators = comparisonOperators + (">" -> gtOperator)
    comparisonOperators = comparisonOperators + (">=" -> gteOperator)
    comparisonOperators = comparisonOperators + ("<" -> ltOperator)
    comparisonOperators = comparisonOperators + ("<=" -> lteOperator)

    val incOperator = (registerValue: Int, value: Int) => (registerValue + value)
    val decOperator = (registerValue: Int, value: Int) => (registerValue - value)
    mutationOperators = mutationOperators + ("inc" -> incOperator)
    mutationOperators = mutationOperators + ("dec" -> decOperator)

    val (registry, maxEver) = processLine(scanner, Map(), 0)
    println("Max value in registry: " + registry.values.foldLeft(registry.valuesIterator.next) { case (a, e) => max(a,e)})
    println("Max value every: " + maxEver)

  }

  def max(a: Int, b: Int): Int = {
    if (a > b) a else b
  }

  def processLine(scanner: Scanner, registry: Map[String, Int], runningMax: Int): (Map[String, Int], Int) = {
    if (!scanner.hasNext()) {
      return (registry, runningMax)
    }

    val mutateRegister = scanner.next()
    println("mutateRegister: " + mutateRegister)
    val mutateOperator = scanner.next()
    println("mutateOperator: " + mutateOperator)
    val mutateOperand = scanner.next().toInt
    println("mutateOperand: " + mutateOperand)
    assert (scanner.next() == "if", {println("Expected 'if' but got something else")}) // consume "if"
    val compareRegister = scanner.next()
    println("compareRegister: " + compareRegister)
    val compareOperator = scanner.next()
    println("compareOperator: " + compareOperator)
    val compareOperand = scanner.next().toInt
    println("compareOperand: " + compareOperand)

    var newRegistry = registry
    val cOp = comparisonOperators.get(compareOperator).get
    val mOp = mutationOperators.get(mutateOperator).get
    var newValue = 0

    if (cOp(registry.getOrElse(compareRegister, 0), compareOperand)) {
      newValue = mOp(registry.getOrElse(mutateRegister, 0), mutateOperand)
      newRegistry = registry + (mutateRegister -> newValue)
    }

    return processLine(scanner, newRegistry, max(runningMax, newValue))
  }
}
