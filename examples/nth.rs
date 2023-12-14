use light_id::LightId;

/**
 * Output:
 * a = a
 * 
 * **/
fn main () {
  let mut generator = LightId::new();

  generator.increment_by(10);

  println!("{} = {}", generator.current(), generator.nth(10));
}