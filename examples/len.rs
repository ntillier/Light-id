use light_id::LightId;

/**
 * Output:
 * 1
 * 21
 * 
 * **/
fn main () {
  let mut generator = LightId::new();

  println!("{}", generator.len());

  generator.min(21);

  println!("{}", generator.len());
}