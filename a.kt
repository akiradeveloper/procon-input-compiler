fun main() {
    val line: String = readLine()!!;
    val xs: List<String> = line.split(' ');
    println(xs);
    println(xs.size);
    val i: Int = 1;
    val f: Double = 1.0;
    val arr = ArrayList<Double>();
    for (i in 0 until 3) {
        // arr.add(xs[i].toInt());
        arr.add(xs[i].toDouble());
    }
    println(arr);
    println("Hello, World!")
}