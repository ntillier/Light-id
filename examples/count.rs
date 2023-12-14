use light_id::LightId;

/**
 * Output:
 * 10
 * 
 * **/
fn main () {
  let mut generator = LightId::new();

  generator.increment_by(10);

  println!("{}", generator.count());
}