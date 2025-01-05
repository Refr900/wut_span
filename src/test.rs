#[test]
fn test() {
    use crate::{Location, Span};
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
