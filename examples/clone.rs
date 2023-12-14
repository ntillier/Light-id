use light_id::LightId;

/**
 * Output:
 * 0 - 0
 * 1 - 1
 * 2 - 2
 * 3 - 3
 * 4 - 4
 * 5 - 5
 * 6 - 6
 * 7 - 7
 * 8 - 8
 * 9 - 9
 * 
 * **/
fn main () {
  let mut generator = LightId::new();
  let mut clone = generator.clone();

  for _ in 0..10 {
    println!("{} - {}", generator.next(), clone.next());
  }
}