pub fn run_generic_in_methods() {
    generic_in_method();
    mix_generic_type();
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


fn generic_in_method() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

// We can also specify constraints on generic types when defining methods on the type.
// In this example, we use the concrete type f32, meaning we don’t declare any types after impl.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Generic type parameters in a struct definition aren’t always the same as those you use in that same struct’s method signatures.
// The method creates a new `Point1` instance with the `x` value from the `self` `Point1` (of type `X1`)
// and the `y` value from the passed-in `Point1` (of type `Y2`).
struct Point1<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point1<X1, Y1> {
    fn mixup<X2, Y2>(
        self,
        other: Point1<X2, Y2>,
    ) -> Point1<X1, Y2> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

fn mix_generic_type() {
    let p1 = Point1 { x: 5, y: 10.4 };
    let p2 = Point1 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("Mix up result: p3.x = {}, p3.y = {}", p3.x, p3.y);
}