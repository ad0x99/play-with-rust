mod destructuring;
mod ignore_values_in_pattern;
mod pattern_syntax;

fn main() {
    pattern_syntax::run_pattern_syntax();
    destructuring::run_destructuring();
    ignore_values_in_pattern::run_ignore_values();
}
