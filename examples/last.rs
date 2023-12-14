use light_id::LightId;

/**
 * Output:
 * abcdef
 * abcdeg
 * abcdeh
 * abcdei
 * abcdej
 * abcdek
 * abcdel
 * abcdem
 * abcden
 * abcdeo
 * 
 * **/
fn main () {
  let mut generator = LightId::new();

  generator.last("abcdef");

  for _ in 0..10 {
    println!("{}", generator.next());
  }
}