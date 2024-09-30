use dyarr::Dyarr;

#[test]
fn basic_use() {
    // this is a 3d array of dimensions [2, 3, 5]
    let array_3d = Dyarr::new(42, &[2, 3, 5]);
    // all element should be 42
    for i in 0..2 {
        for j in 0..3 {
            for k in 0..5 {
                // use [usize] or &[usize] as index to find an element
                assert_eq!(array_3d[[i, j, k]], 42);
            }
        }
    }
    // setting value is also supported
    let mut array_3d = array_3d;
    array_3d[[1, 0, 2]] = -array_3d[[1, 0, 2]];
    assert_eq!(array_3d[[1, 0, 2]], -42);
}

#[test]
#[should_panic]
fn indices_length_is_checked() {
    let array_3d = Dyarr::new(42, &[2, 3, 5]);
    let _ = array_3d[[1, 2]];
}

#[test]
#[should_panic]
fn index_bound_is_checked() {
    let array_3d = Dyarr::new(42, &[2, 3, 5]);
    let _ = array_3d[[0, 1, 6]];
}

#[test]
fn interact_with_slice_underneath() {
    // you can construct the structure by giving a boxed slice
    // representing data which is in row-major order,
    // and the dimensions.
    let data: Vec<_> = (0..24).collect();
    let array_3d = Dyarr::from_raw_parts(
        data.clone() /* reused later */
            .into(),
        [2, 3, 4].into(),
    )
    .unwrap();
    // dimensions are checked, that's why return value is Result
    Dyarr::from_raw_parts(data.into(), [3, 4, 5].into()).unwrap_err();

    // and to get raw slice, call .raw() .raw_ref() or .raw_mut()
    assert_eq!(array_3d.raw_ref()[7], 7);

    // to get dimensions info, use .dim()
    assert_eq!(array_3d.dim().len(), 3);
}

#[test]
fn calculate_index_in_raw_slice() {
    let array_3d = Dyarr::new(0, &[2, 3, 4]);
    // if you want to know where an element is in raw slice,
    // you can use .offset()
    assert_eq!(array_3d.offset(&[1, 2, 3]).unwrap(), 23);
    // .offset() also accepts negative index
    assert_eq!(array_3d.offset(&[-1, -2, -3]).unwrap(), -23);
    // and it accepts shorter demension array
    // higher index is default to 0
    assert_eq!(array_3d.offset(&[3]).unwrap(), 3);
    // however there is still some checks
    // every index should in range [1 - len_of_row, len_of_row - 1]
    array_3d.offset(&[0, 0, 4]).unwrap_err();
}

// for more details and all interfaces please see the source code.