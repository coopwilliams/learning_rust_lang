

fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    
    let mut v1_iter = v1.iter();
    
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

fn iterator_sum() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

fn iterator_adaptor() {
    let v1 = vec![1,2,3];
    // map() is lazy, so we have to use it with collect();
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2,3,4]);
    println!("{:?}", v1);
}

fn main() {
    iterator_demonstration();
    iterator_sum();
    iterator_adaptor();
}
