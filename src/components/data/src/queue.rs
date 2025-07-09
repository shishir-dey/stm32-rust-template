use core::ptr::drop_in_place;

pub struct Queue<T, const N: usize> {
    buf: [core::mem::MaybeUninit<T>; N],
    head: usize,
    tail: usize,
    len: usize,
}

impl<T, const N: usize> Queue<T, N> {
    /// Create a new, empty queue.
    pub const fn new() -> Self {
        // SAFETY: An uninitialized array of MaybeUninit is valid.
        Self {
            buf: unsafe { core::mem::MaybeUninit::uninit().assume_init() },
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    /// Returns the capacity of the queue.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns the number of elements in the queue.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns true if the queue is full.
    pub fn is_full(&self) -> bool {
        self.len == N
    }

    /// Adds an element to the queue.
    /// Returns Err if the queue is full.
    pub fn enqueue(&mut self, item: T) -> Result<(), T> {
        if self.is_full() {
            return Err(item);
        }
        self.buf[self.tail].write(item);
        self.tail = (self.tail + 1) % N;
        self.len += 1;
        Ok(())
    }

    /// Removes and returns the front element of the queue.
    /// Returns None if the queue is empty.
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let item = unsafe { self.buf[self.head].as_ptr().read() };
        self.head = (self.head + 1) % N;
        self.len -= 1;
        Some(item)
    }

    /// Returns a reference to the front element, if any.
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            // SAFETY: We know there's an element at head.
            Some(unsafe { &*self.buf[self.head].as_ptr() })
        }
    }

    /// Returns a mutable reference to the front element, if any.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            // SAFETY: We know there's an element at head.
            Some(unsafe { &mut *self.buf[self.head].as_mut_ptr() })
        }
    }
}

impl<T, const N: usize> Drop for Queue<T, N> {
    fn drop(&mut self) {
        // Drop all initialized elements
        let mut idx = self.head;
        for _ in 0..self.len {
            unsafe {
                drop_in_place(self.buf[idx].as_mut_ptr());
            }
            idx = (idx + 1) % N;
        }
    }
}
