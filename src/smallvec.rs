#![allow(dead_code)]
use std::mem::{self, MaybeUninit};
use std::ops::{Index, IndexMut};
use std::ptr;

pub union Storage<T, const N: usize>
where
    T: Sized + Copy,
{
    stack: [mem::MaybeUninit<T>; N],
    heap: (*mut T, usize),
}

pub struct SmallVec<T, const E: usize>
where
    T: Sized + Copy,
{
    store: Storage<T, E>,
    is_heap: bool,
    len: usize,
}

impl<T, const N: usize> SmallVec<T, N>
where
    T: Sized + Copy,
{
    pub fn new() -> Self {
        Self {
            store: Storage {
                stack: unsafe { mem::MaybeUninit::uninit().assume_init() },
            },
            is_heap: false,
            len: 0,
        }
    }

    pub fn push(&mut self, el: T) {
        if self.len == N {
            self.move_to_heap();
        }

        let len = self.len;
        self.as_mut_slice()[len] = el;
        self.len += 1;
    }

    #[rustfmt::skip]
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "Index out of bounds");

        let item = unsafe { ptr::read(self.as_ptr().add(index)) };

        if index < self.len - 1 {
            unsafe {
                ptr::copy(
                    self.as_ptr().add(index + 1),
                    self.as_mut_ptr().add(index),
                    self.len - index - 1,
                )
            };
        }

        self.len -= 1;

        let cap = self.capacity();
        if self.len <= N && cap > N {
            let stack = unsafe {
                let mut maybe: [MaybeUninit<T>; N] = MaybeUninit::uninit().assume_init();
                ptr::copy_nonoverlapping(
                    self.as_ptr(),
                    maybe.as_mut_ptr() as *mut T,
                    self.len
                );
                maybe
            };

            unsafe {
                std::alloc::dealloc(
                    self.store.heap.0 as *mut u8,
                    std::alloc::Layout::array::<T>(self.store.heap.1).unwrap(),
                );
            }

            self.store = Storage { stack };
        }

        item
    }

    pub fn capacity(&self) -> usize {
        if self.len < N {
            N
        } else {
            unsafe { self.store.heap.1 }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn move_to_heap(&mut self) {
        if self.len <= N {
            let mut heap = Vec::<T>::with_capacity(N * 2);
            heap.extend_from_slice(self.as_mut_slice());
            self.store = Storage {
                heap: (heap.as_mut_ptr(), heap.capacity()),
            };
            self.is_heap = true;
            mem::forget(heap);
        }
    }

    fn as_ptr(&self) -> *const T {
        if self.len < N {
            unsafe { self.store.stack.as_ptr() as *const T }
        } else {
            unsafe { self.store.heap.0 as *const T }
        }
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        if self.len < N {
            unsafe { self.store.stack.as_mut_ptr() as *mut T }
        } else {
            unsafe { self.store.heap.0 as *mut T }
        }
    }

    fn as_slice(&self) -> &[T] {
        if self.is_heap {
            unsafe { std::slice::from_raw_parts(self.store.heap.0, self.len) }
        } else {
            unsafe { mem::transmute(self.store.stack.as_slice()) }
        }
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        if self.is_heap {
            unsafe { std::slice::from_raw_parts_mut(self.store.heap.0, self.capacity()) }
        } else {
            unsafe { mem::transmute(self.store.stack.as_mut()) }
        }
    }
}

impl<T, const N: usize> From<[T; N]> for SmallVec<T, N>
where
    T: Sized + Copy,
{
    fn from(arr: [T; N]) -> Self {
        let mut vec: SmallVec<T, N> = SmallVec::new();
        unsafe { ptr::copy_nonoverlapping(arr.as_ptr(), vec.as_mut_ptr(), N) }
        vec.len = arr.len();
        vec
    }
}

impl<'a, T, const N: usize> From<&'a mut [T]> for SmallVec<T, N>
where
    T: Sized + Copy,
{
    fn from(slice: &'a mut [T]) -> Self {
        let len = slice.len();
        let vec = if len < N {
            SmallVec {
                store: Storage {
                    stack: unsafe {
                        let mut stack: [MaybeUninit<T>; N] = MaybeUninit::uninit().assume_init();
                        ptr::copy_nonoverlapping(slice.as_ptr(), stack.as_mut_ptr() as *mut T, len);
                        stack
                    },
                },
                is_heap: false,
                len,
            }
        } else {
            let mut small = SmallVec::<T, N>::new();
            let layout = std::alloc::Layout::array::<T>(len).unwrap();
            unsafe {
                let ptr = std::alloc::alloc(layout);
                small.store = Storage {
                    heap: (ptr as *mut T, len),
                };
                std::ptr::copy_nonoverlapping(slice.as_ptr(), small.store.heap.0, len);
                small.len = len;
                small.is_heap = true;
            }
            small
        };
        vec
    }
}

// impl<T, const N: usize> From<Vec<T>> for SmallVec<T, N>
// where
//     T: Sized + Copy,
// {
//     fn from(vec: Vec<T>) -> Self {
//         let mut small_vec = SmallVec::new();
//         // let mut heap = Vec::<T>::with_capacity(N * 2);
//         // vec.extend_from_slice(small_vec.as_mut_slice());
//         small_vec.store = Storage {
//             heap: (vec.as_mut_ptr(), vec.capacity()),
//         };
//         mem::forget(vec);
//         small_vec
//     }
// }

impl<T, const N: usize> Index<usize> for SmallVec<T, N>
where
    T: Sized + Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for SmallVec<T, N>
where
    T: Sized + Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl<T, const N: usize> Drop for SmallVec<T, N>
where
    T: Sized + Copy,
{
    fn drop(&mut self) {
        if self.is_heap {
            unsafe {
                ptr::drop_in_place(&mut self.store.heap);
            }
        } else {
            for i in 0..self.len {
                unsafe {
                    ptr::drop_in_place(self.store.stack[i].as_mut_ptr());
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::SmallVec;

    #[test]
    fn test_empty() {
        let v = SmallVec::<usize, 0>::new();
        assert!(!v.is_heap);
        assert!(v.capacity() == 0);
        assert!(v.is_empty());
    }

    #[test]
    fn test_from_array() {
        let v = SmallVec::from([1, 2, 3]);
        assert!(!v.is_heap);
        assert!(v.capacity() == 3);
        assert!(!v.is_empty());
        assert!(v[0] == 1);
        assert!(v[1] == 2);
        assert!(v[2] == 3);
    }

    #[test]
    fn test_move_to_heap() {
        let mut v = SmallVec::from([1, 2, 3]);
        assert!(!v.is_heap);
        v.push(4);
        assert!(v.is_heap);
        assert!(v.capacity() == 6);
        assert!(v.len == 4);
        assert!(!v.is_empty());
        assert!(v[0] == 1);
        assert!(v[1] == 2);
        assert!(v[2] == 3);
        assert!(v[3] == 4);
    }
}
