use light_id::{LightId, IdSwitcher, DEFAULT_CHARACTERS};

/**
 * Output:
 * 1C -> aaabacab
 * 1C <- aaabacab
 * #100 = aaabacab
 * #100 = 1C
 * 
 * **/

 fn main () {
  let mut generator = LightId::new();
  let mut switcher = IdSwitcher::new(DEFAULT_CHARACTERS, "abc");

  switcher.min_target(8);
  switcher.min_source(2);

  generator.increment_by(100);

  let reversed = switcher.switch(generator.current());

  println!("{} -> {}", generator.current(), reversed);

  println!("{} <- {}", switcher.switch_reverse(&reversed), reversed);

  println!("#100 = {}", switcher.switch_count(100));
  println!("#100 = {}", switcher.switch_count_reverse(100));
}