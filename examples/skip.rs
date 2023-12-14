use light_id::LightId;

/**
 * Output:
 * 2Bi
 * 2Bj
 * 2Bk
 * 2Bl
 * 2Bm
 * 2Bn
 * 2Bo
 * 2Bp
 * 2Bq
 * 2Br
 * 
 * **/
fn main () {
  let mut generator = LightId::new();

  generator.skip(10000);

  for _ in 0..10 {
    println!("{}", generator.next());
  }
}