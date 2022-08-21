extern crate buffered_vec;

#[test]
fn push_to_bunch() {
    let size = 100000usize;

    let mut bunch = buffered_vec::Bunch::<usize>::new(size);

    for i in 0 as usize..size {
        bunch.push(i).unwrap();
    }

    for i in 0 as usize..size {
        assert_eq!(bunch.data[i], i);
    }

    assert_eq!(bunch.filled, size);
}

#[test]
fn pop_from_bunch() {
    let size = 100000usize;

    let mut bunch = buffered_vec::Bunch::<usize>::new(size);

    for i in 0 as usize..size {
        bunch.push(i).unwrap();
    }

    for i in (0 as usize..size).rev() {
        assert_eq!(bunch.pop().unwrap(), i);
    }

    assert_eq!(bunch.filled, 0);
}

#[test]
fn set_bunch() {
    let size = 100000usize;

    let mut bunch = buffered_vec::Bunch::<usize>::new(size);

    for i in 0 as usize..size {
        bunch.push(i).unwrap();
    }

    for i in (0 as usize..size).rev() {
        bunch.set(i, 255).unwrap();
    }

    assert_eq!(bunch.filled, size);

    for _ in 0 as usize..size {
        assert_eq!(bunch.pop().unwrap(), 255);
    }

    assert_eq!(bunch.filled, 0);
}

#[test]
fn get_from_bunch() {
    let size = 100000usize;

    let mut bunch = buffered_vec::Bunch::<usize>::new(size);

    for i in 0 as usize..size {
        bunch.push(i).unwrap();
    }

    for i in 0 as usize..size {
        assert_eq!(bunch.get(i).unwrap(), i);
    }

    assert_eq!(bunch.filled, size);
}

#[test]
fn remove_from_bunch() {
    let size = 100000usize;

    let mut bunch = buffered_vec::Bunch::<usize>::new(size);

    for i in 0 as usize..size {
        bunch.push(i).unwrap();
    }

    assert_eq!(bunch.remove(50).unwrap(), 50);
    assert_eq!(bunch.filled, size - 1);
    assert_ne!(bunch.get(50).unwrap(), 50);

    for i in 0 as usize..50 {
        assert_eq!(bunch.get(i).unwrap(), i);
    }

    for i in 50 as usize..size - 1 {
        assert_eq!(bunch.get(i).unwrap(), i + 1);
    }
}
