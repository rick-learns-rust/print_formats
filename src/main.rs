use std::fmt;

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        write!(f, "{}", self.0)
    }
}

fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Different formatting can be invoked by specifying the format character after a
    // `:`.
    let width_label = 25;
    let width_number = 20;
    println!("{:.<width_label$}{:0>width_number$}", "Base 10:", 69420);
    println!(
        "{:<width_label$}{:>width_number$b}",
        "Base 2 (binary):", 69420
    );
    println!(
        "{:<width_label$}{:>width_number$o}",
        "Base 8 (octal):", 69420
    );
    println!(
        "{:<width_label$}{:>width_number$x}",
        "Base 16 (hexadecimal):", 69420
    );
    println!(
        "{:<width_label$}{:>width_number$X}",
        "Base 16 (hexadecimal):", 69420
    );

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes,
    //and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number = 1);

    // You can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    let value = 7;
    println!("This struct \n`{:?}`\n will print...", Structure(value));
    println!("This struct \n`{}`\n will print...", Structure(value));

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "     1". 5 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("{:0>7.3}", pi);
}
