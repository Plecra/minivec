extern crate minivec;

use minivec::mini_vec;
use minivec::MiniVec;

#[test]
fn minivec_default_constructed() {
  let v: MiniVec<i32> = MiniVec::new();
  assert_eq!(v.capacity(), 0);
  assert_eq!(v.len(), 0);
  assert!(v.is_empty());

  let v: MiniVec<i32> = Default::default();
  assert_eq!(v.capacity(), 0);
  assert_eq!(v.len(), 0);
  assert!(v.is_empty());
}

#[test]
fn minivec_push() {
  let mut v: MiniVec<i32> = MiniVec::new();

  assert_eq!(v.len(), v.capacity());

  v.push(1);
  v.push(2);
  v.push(3);

  assert_eq!(v.len(), 3);
  assert!(v.capacity() >= v.len());

  let mut v: MiniVec<String> = MiniVec::new();

  assert_eq!(v.len(), v.capacity());

  v.push(String::from("Hello"));
  v.push(String::from("Rust"));
  v.push(String::from("World!"));

  assert_eq!(v.len(), 3);
  assert!(v.capacity() >= v.len());

  let mut v: MiniVec<String> = MiniVec::new();

  assert_eq!(v.len(), v.capacity());

  for _ in 0..32 {
    v.push(String::from("Hello, world!"));
  }

  assert_eq!(v.len(), 32);
  assert!(v.capacity() >= v.len());
}

#[test]
fn minivec_deref_test() {
  let mut v: MiniVec<i32> = MiniVec::new();
  v.push(1);
  v.push(2);
  v.push(3);

  assert_eq!(v[0], 1);
  assert_eq!(v[1], 2);
  assert_eq!(v[2], 3);
}

#[test]
fn minivec_dedup_by_test() {
  let mut v = mini_vec![1, 2, 1, 1, 3, 3, 3, 4, 5, 4];
  v.dedup_by(|x, y| x == y);

  assert_eq!(v, [1, 2, 1, 3, 4, 5, 4]);
}

#[test]
fn minivec_dedup_needs_drop() {
  let mut v: MiniVec<Box<_>> = mini_vec![Box::new(1), Box::new(1), Box::new(2), Box::new(3)];
  v.dedup();

  assert_eq!(v.len(), 3);
}

#[test]
fn minivec_with_capacity() {
  let size = 128;
  let mut v: MiniVec<i32> = MiniVec::with_capacity(size);

  assert_eq!(v.len(), 0);
  assert_eq!(v.capacity(), size);

  v.push(1);
  v.push(2);
  v.push(3);

  assert_eq!(v.len(), 3);
  assert_eq!(v.capacity(), size);
}

#[test]
fn minivec_extend_from_slice() {
  let a = [2, 3, 4, 5];
  let mut v = mini_vec![1];
  v.extend_from_slice(&a);

  assert_eq!(v.len(), 5);

  v.extend_from_slice(&[6, 7, 8]);
  assert_eq!(v.len(), 8);

  assert_eq!(a.len(), 4);
  assert_eq!(a, [2, 3, 4, 5]);

  let a: MiniVec<_> = [2, 3, 4, 5].iter().map(|x| x.to_string()).collect();
  let mut v = mini_vec![String::from("1")];
  v.extend_from_slice(&a);

  assert_eq!(v.len(), 5);

  v.extend_from_slice(&[6.to_string(), 7.to_string(), 8.to_string()]);
  assert_eq!(v.len(), 8);

  assert_eq!(a.len(), 4);
  assert_eq!(
    a,
    [2.to_string(), 3.to_string(), 4.to_string(), 5.to_string()]
  );
}

#[test]
fn minivec_from_raw_part() {
  use std::{mem, ptr};

  let v = mini_vec![1, 2, 3];
  let mut v = mem::ManuallyDrop::new(v);

  let p = v.as_mut_ptr();
  let len = v.len();
  let cap = v.capacity();

  unsafe {
    for i in 0..len as isize {
      ptr::write(p.offset(i), 4 + i);
    }

    let rebuilt = MiniVec::from_raw_part(p);
    assert_eq!(rebuilt, [4, 5, 6]);
    assert_eq!(rebuilt.capacity(), cap);
    assert_eq!(rebuilt.len(), len);
  }
}

#[test]
fn minivec_from_raw_parts() {
  use std::{mem, ptr};

  let v = mini_vec![1, 2, 3];
  let mut v = mem::ManuallyDrop::new(v);

  let p = v.as_mut_ptr();
  let len = v.len();
  let cap = v.capacity();

  unsafe {
    for i in 0..len as isize {
      ptr::write(p.offset(i), 4 + i);
    }

    let rebuilt = MiniVec::from_raw_parts(p, len, cap);
    assert_eq!(rebuilt, [4, 5, 6]);
    assert_eq!(rebuilt.capacity(), cap);
    assert_eq!(rebuilt.len(), len);
  }
}

#[test]
fn minivec_insert() {
  let mut vec = mini_vec![1, 2, 3];

  vec.insert(1, 4);
  assert_eq!(vec, [1, 4, 2, 3]);

  vec.insert(4, 5);
  assert_eq!(vec, [1, 4, 2, 3, 5]);

  let mut vec: MiniVec<String> = mini_vec![1.to_string(), 2.to_string(), 3.to_string()];

  vec.insert(1, 4.to_string());
  assert_eq!(
    vec,
    [1.to_string(), 4.to_string(), 2.to_string(), 3.to_string()]
  );

  vec.insert(4, 5.to_string());
  assert_eq!(
    vec,
    [
      1.to_string(),
      4.to_string(),
      2.to_string(),
      3.to_string(),
      5.to_string()
    ]
  );
}

#[test]
fn minivec_leak() {
  let x = mini_vec![1, 2, 3];
  let static_ref: &'static mut [usize] = MiniVec::leak(x);
  static_ref[0] += 1;
  assert_eq!(static_ref, &[2, 2, 3]);
}

#[test]
fn minivec_pop() {
  let mut vec = mini_vec![1, 2, 3];
  assert_eq!(vec.pop(), Some(3));
  assert_eq!(vec, [1, 2]);

  let mut vec = mini_vec![1.to_string(), 2.to_string(), 3.to_string()];
  assert_eq!(vec.pop(), Some(String::from("3")));
  assert_eq!(vec, [String::from("1"), String::from("2")]);
}

#[test]
fn minivec_remove() {
  let mut v = mini_vec![1, 2, 3];
  assert_eq!(v.remove(1), 2);
  assert_eq!(v, [1, 3]);
}

#[test]
#[should_panic]
fn minivec_remove_panic() {
  let mut v = mini_vec![1, 2, 3];
  v.remove(v.len());
}
