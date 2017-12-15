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

    println(processLine(scanner, Map()))
  }

  def processLine(scanner: Scanner, registry: Map[String, Int]): Map[String, Int] = {
    if (!scanner.hasNext()) {
      return registry
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

    var new_registry = registry
    val cOp = comparisonOperators.get(compareOperator).get
    val mOp = mutationOperators.get(mutateOperator).get

    if (cOp(registry.getOrElse(compareRegister, 0), compareOperand)) {
      new_registry = registry + (mutateRegister -> mOp(registry.getOrElse(mutateRegister, 0), mutateOperand))
    }

    return processLine(scanner, new_registry)
  }
}
