use light_id::LightId;

/**
 * Output:
 * a
 * b
 * c
 * d
 * e
 * f
 * ba
 * bb
 * bc
 * bd
 * --- toogle charset ---
 * 1b
 * 1c
 * 20
 * 21
 * 22
 * 2a
 * 2b
 * 2c
 * a0
 * a1
 * 
 * **/
fn main () {
  let mut generator = LightId::from("abcdef");

  for _ in 0..10 {
    println!("{}", generator.next());
  }

  println!("--- toogle charset ---");

  // We toogle the charset (without resetting the count)
  generator.chars("012abc");

  for _ in 0..10 {
    println!("{}", generator.next());
  }
}