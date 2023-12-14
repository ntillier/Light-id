use light_id::LightId;

/**
 * Output:
 * 10
 * 1000
 * 
 * **/
fn main () {
  let mut generator = LightId::new();

  generator.increment_by(10);
  
  println!("{}", generator.index(generator.current()));

  println!("{}", generator.index(generator.nth(1000)));
}