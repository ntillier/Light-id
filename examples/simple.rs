use light_id::LightId;

/**
 * Output:
 * 0
 * 1
 * 2
 * 3
 * 4
 * 5
 * 6
 * 7
 * 8
 * 9
 * 
 * **/
fn main () {
  let mut generator = LightId::new();

  for _ in 0..10 {
    println!("{}", generator.next());
  }
}