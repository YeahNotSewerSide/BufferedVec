extern crate buffered_vec;

const SIZE: usize = 10000;

#[test]
fn push_to_bunch() {
    let mut bunch = buffered_vec::Bunch::<usize>::new(SIZE);

    for i in 0 as usize..SIZE {
        bunch.push(i).unwrap();
    }

    for i in 0 as usize..SIZE {
        assert_eq!(bunch.get(i).unwrap(), i);
    }

    assert_eq!(bunch.filled(), SIZE);
}

#[test]
fn pop_from_bunch() {
    let mut bunch = buffered_vec::Bunch::<usize>::new(SIZE);

    for i in 0 as usize..SIZE {
        bunch.push(i).unwrap();
    }

    for i in (0 as usize..SIZE).rev() {
        assert_eq!(bunch.pop().unwrap(), i);
    }

    assert_eq!(bunch.filled(), 0);
}

#[test]
fn set_bunch() {
    let mut bunch = buffered_vec::Bunch::<usize>::new(SIZE);

    for i in 0 as usize..SIZE {
        bunch.push(i).unwrap();
    }

    for i in (0 as usize..SIZE).rev() {
        bunch.set(i, 255).unwrap();
    }

    assert_eq!(bunch.filled(), SIZE);

    for _ in 0 as usize..SIZE {
        assert_eq!(bunch.pop().unwrap(), 255);
    }

    assert_eq!(bunch.filled(), 0);
}

#[test]
fn get_from_bunch() {
    let mut bunch = buffered_vec::Bunch::<usize>::new(SIZE);

    for i in 0 as usize..SIZE {
        bunch.push(i).unwrap();
    }

    for i in 0 as usize..SIZE {
        assert_eq!(bunch.get(i).unwrap(), i);
    }

    assert_eq!(bunch.filled(), SIZE);
}

#[test]
fn remove_from_bunch() {
    let mut bunch = buffered_vec::Bunch::<usize>::new(SIZE);

    for i in 0 as usize..SIZE {
        bunch.push(i).unwrap();
    }

    let index = SIZE / 2;

    assert_eq!(bunch.remove(index).unwrap(), index);
    assert_eq!(bunch.filled(), SIZE - 1);
    assert_ne!(bunch.get(index).unwrap(), index);

    for i in 0 as usize..index {
        assert_eq!(bunch.get(i).unwrap(), i);
    }

    for i in index as usize..SIZE - 1 {
        assert_eq!(bunch.get(i).unwrap(), i + 1);
    }
}

#[test]
fn insert_to_bunch() {
    let mut bunch = buffered_vec::Bunch::<usize>::new(SIZE);

    for i in 0..SIZE * 2 {
        bunch.insert(0, i).unwrap();
    }

    for i in SIZE..SIZE * 2 {
        assert_eq!(bunch.pop().unwrap(), i);
    }
}

#[test]
fn push_vec() {
    let mut buf_vec = buffered_vec::BufferedVec::<usize>::new(SIZE);

    for i in 0..SIZE * 2 {
        buf_vec.push(i);
    }

    assert_eq!(buf_vec.len(), SIZE * 2);

    buf_vec.push(1);

    println!("{}", buf_vec.capacity());

    //println!("{:?}", buf_vec);
}

#[test]
fn pop_from_vec() {
    let mut buf_vec = buffered_vec::BufferedVec::<usize>::with_capacity(SIZE, SIZE * 4);

    for i in 0..SIZE * 3 {
        buf_vec.push(i);
    }

    for i in (0..SIZE * 3).rev() {
        assert_eq!(buf_vec.pop().unwrap(), i);
    }

    assert_eq!(buf_vec.len(), 0);
}

#[test]
fn get_from_vec() {
    let mut buf_vec = buffered_vec::BufferedVec::<usize>::with_capacity(SIZE, SIZE * 4);

    for i in 0..SIZE * 2 {
        buf_vec.push(i);
    }

    for i in 0..SIZE * 2 {
        assert_eq!(buf_vec.get(i), Some(i));
    }

    assert_eq!(buf_vec.get(SIZE * 2), None);

    let mut buf_vec = buffered_vec::BufferedVec::<usize>::with_capacity(SIZE, SIZE * 4);
    assert_eq!(buf_vec.get(1000), None);

    for i in 0..SIZE {
        buf_vec.push(i);
    }

    for i in 0..SIZE {
        assert_eq!(buf_vec.get(i), Some(i));
    }
}
