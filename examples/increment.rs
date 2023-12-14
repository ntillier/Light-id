use light_id::LightId;

/**
 * Output:
 * 0
 * 1
 * b
 * 1
 * 0
 * 
 * **/
fn main () {
  let mut generator = LightId::new();

  println!("{}", generator.current());

  generator.increment();
  println!("{}", generator.current());
  
  generator.increment_by(10);
  println!("{}", generator.current());
  
  generator.decrement_by(10);
  println!("{}", generator.current());
  
  generator.decrement();
  println!("{}", generator.current());
}