use abcgraphs;

#[test]
fn sum_positive() {
    assert_eq!(3, abcgraphs::add(1, 2));
    assert_eq!(4, abcgraphs::add(2, 2));
    assert_eq!(5, abcgraphs::add(5, 0));
}
