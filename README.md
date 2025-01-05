> [!WARNING]
> The project was created solely for educational purposes.

# Wut Span

A small crate for counting rows and columns in source text. And also to highlight part of the text around a specific location(in development, Oopsie).

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
wut_span = { version = "0.1.0", git = "https://github.com/Refr900/wut_span"}
```

Then, you can use it:

```rust
use wut_span::{Location, Span};

fn main() {
    let source = "
// one staff
let x = (10 + 6) / 2 - 2
// other staff
    "
    .trim_start();
    // `(10 + 6) / 2 - 2`
    let span = Span::new(21, 37);

    assert!(
        span.is_contained_in(source),
        "source does not include span!"
    );

    assert!(
        // you can convert from span to location and again to span
        span == span.to_location(source).to_span(source),
        "conversation is wrong!"
    );

    assert!(
        Location::from_span(source, span) == Location::new(2, 9, 16),
        "location is wrong!"
    );
}
```

## License

See the [LICENSE](LICENSE) file for license rights and limitations (MIT).
