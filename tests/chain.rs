
#[macro_use] extern crate utils;

#[test]
fn chain() {

    let empty: ::std::iter::Empty<i32> = chain![];
    let empty_vec: Vec<i32> = vec![];
    assert_eq!(empty.collect::<Vec<_>>(), empty_vec);

    let one = chain![Some("G'day")];
    assert_eq!(one.collect::<Vec<_>>(), vec!["G'day"]);

    let nums = chain![Some(0), vec![1, 2, 3, 4], Some(5).into_iter().chain(Some(6))];
    assert_eq!(nums.collect::<Vec<_>>(), vec![0, 1, 2, 3, 4, 5, 6]);

}

