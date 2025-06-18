fn main() {
    // Comparison (lexicographic order)
let tuple1 = (1, 1);
let tuple2 = (1, 1);
let tuple3 = (1, 3);

println!("{}", tuple1 == tuple2);           // true (3 < 4)
println!("{}", tuple1 < tuple3);           // true (2 < 3 at second position)
// println!("{}", tuple1 == tuple1);  
}
