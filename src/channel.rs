//!
//!  channel.rs
//!
//!  Created by Mitchell Nordine at 02:00PM on March 25, 2015.
//!
//!


/// A module for a channel that acts exactly as std::sync::mpsc::channel does, but rather than
/// storing messages in an underlyhing queue, it only stores the latest message.
pub mod last {
    use std::cell::UnsafeCell;
    use std::sync::{Arc, TryLockError, Mutex};

    /// For sending the latest value.
    pub struct Sender<T: Send> {
        data: UnsafeCell<Arc<UnsafeCell<Arc<Mutex<Option<T>>>>>>,
    }

    impl<T: Send> Clone for Sender<T> {
        fn clone(&self) -> Sender<T> {
            let new_data_ptr = unsafe { (*self.data.get()).clone() };
            Sender {
                data: UnsafeCell::new(new_data_ptr),
            }
        }
    }

    /// We need an unsafe implementation of Send because of the UnsafeCell.
    unsafe impl<T> Send for Sender<T> {}

    /// The Receiver was dropped and the channel was closed.
    pub struct SendError<T>(T);

    /// An enumeration of the different possible try_send errors.
    pub enum TrySendError<T> {
        /// The Receiver was dropped and the channel was closed.
        ChannelClosed(T),
        /// The mutex is currently locked.
        WouldBlock(T),
    }

    impl<T: Send> Sender<T> {

        /// Send the latest value to the receiver.
        /// This may block shortly if the receiver has locked the mutex.
        pub fn send(&self, t: T) -> Result<(), SendError<T>> {
            unsafe {
                let data_ptr = (*self.data.get()).get();
                *data_ptr = match (*data_ptr).downgrade().upgrade() {
                    Some(data) => data,
                    None => return Err(SendError(t)),
                };
                match (*data_ptr).lock() {
                    Ok(mut guard) => *guard = Some(t),
                    Err(_) => return Err(SendError(t)),
                }
                Ok(())
            }
        }

        /// Try and send the latest value to the receiver.
        /// If the mutex is currently locked by the receiver, return an Error indicating so.
        /// This method will never lock.
        pub fn try_send(&self, t: T) -> Result<(), TrySendError<T>> {
            unsafe {
                let data_ptr = (*self.data.get()).get();
                *data_ptr = match (*data_ptr).downgrade().upgrade() {
                    Some(data) => data,
                    None => return Err(TrySendError::ChannelClosed(t)),
                };
                match (*data_ptr).try_lock() {
                    Ok(mut guard) => *guard = Some(t),
                    Err(err) => match err {
                        TryLockError::Poisoned(_) => return Err(TrySendError::ChannelClosed(t)),
                        TryLockError::WouldBlock => return Err(TrySendError::WouldBlock(t)),
                    }
                }
                Ok(())
            }
        }

    }

    /// For receiving the latest value if there has been an update.
    pub struct Receiver<T: Send> {
        data: UnsafeCell<Arc<Mutex<Option<T>>>>,
    }

    /// We need an unsafe implementation of Send because of the UnsafeCell.
    unsafe impl<T: Send> Send for Receiver<T> {}

    /// The different kinds of possible receive errors.
    pub enum RecvError {
        /// The Sender was dropped and the channel was closed.
        ChannelClosed,
        /// There have been no updates to the data since the last receive.
        NoNewValue,
    }

    /// An enumeration of the different possible try_recv errors.
    pub enum TryRecvError {
        /// The Sender was dropped and the channel was closed.
        ChannelClosed,
        /// The sender has acquired the mutex and waiting would block the thread.
        WouldBlock,
        /// There have been no updates to the data since the last receive.
        NoNewValue,
    }

    impl<T: Send> Receiver<T> {

        /// Take the latest value if there is one, otherwise return a RecvError.
        /// This will block if the mutex is locked.
        pub fn recv(&self) -> Result<T, RecvError> {
            unsafe {
                let data_ptr = self.data.get();
                *data_ptr = match (*data_ptr).downgrade().upgrade() {
                    Some(data) => data,
                    None => return Err(RecvError::ChannelClosed),
                };
                match (*data_ptr).lock() {
                    Ok(mut guard) => match guard.take() {
                        Some(t) => Ok(t),
                        None => Err(RecvError::NoNewValue),
                    },
                    Err(_) => Err(RecvError::ChannelClosed),
                }
            }
        }

        /// Attempt to retrieve the latest value.
        /// This will never block.
        pub fn try_recv(&self) -> Result<T, TryRecvError> {
            unsafe {
                let data_ptr = self.data.get();
                *data_ptr = match (*data_ptr).downgrade().upgrade() {
                    Some(data) => data,
                    None => return Err(TryRecvError::ChannelClosed),
                };
                match (*data_ptr).try_lock() {
                    Ok(mut guard) => match guard.take() {
                        Some(t) => Ok(t),
                        None => Err(TryRecvError::NoNewValue),
                    },
                    Err(err) => match err {
                        TryLockError::Poisoned(_) => Err(TryRecvError::ChannelClosed),
                        TryLockError::WouldBlock => Err(TryRecvError::WouldBlock),
                    }
                }
            }
        }

    }

    /// Construct a new sender, receiver pair.
    pub fn channel<T: Send>() -> (Sender<T>, Receiver<T>) {
        let arc = Arc::new(Mutex::new(None));
        let sender_arc = UnsafeCell::new(Arc::new(UnsafeCell::new(arc.clone())));
        (Sender { data: sender_arc }, Receiver { data: UnsafeCell::new(arc) })
    }

}


pub mod last_map {
    use std::cell::UnsafeCell;
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::sync::{Arc, TryLockError, Mutex};

    /// A clonable Sender of a key and value pair.
    pub struct Sender<K: Send, V: Send> {
        data: UnsafeCell<Arc<UnsafeCell<Arc<Mutex<HashMap<K, Option<V>>>>>>>,
    }

    impl<K: Send, V: Send> Clone for Sender<K, V> {
        fn clone(&self) -> Sender<K, V> {
            let new_data_ptr = unsafe { (*self.data.get()).clone() };
            Sender {
                data: UnsafeCell::new(new_data_ptr),
            }
        }
    }

    /// We need an unsafe implementation of Send because of the UnsafeCell.
    unsafe impl<K, V> Send for Sender<K, V> {}

    /// The Receiver was dropped and the channel was closed.
    pub struct SendError<K, V>(K, V);

    /// An enumeration of the different possible try_send errors.
    pub enum TrySendError<K, V> {
        /// The Receiver was dropped and the channel was closed.
        ChannelClosed(K, V),
        /// The mutex is currently locked.
        WouldBlock(K, V),
    }

    impl<K: Send + Hash + Eq, V: Send> Sender<K, V> {

        /// Update the underlying hashmap with the given new key, value pair.
        /// May block briefly when acquiring access to the Mutex if either:
        /// - Another sender is sending data at the same time or
        /// - The receiver is receiving the values.
        pub fn send(&self, key: K, value: V) -> Result<(), SendError<K, V>> {
            unsafe {
                let data_ptr = (*self.data.get()).get();
                *data_ptr = match (*data_ptr).downgrade().upgrade() {
                    Some(data) => data,
                    None => return Err(SendError(key, value)),
                };
                match (*data_ptr).lock() {
                    Ok(mut guard) => {
                        guard.insert(key, Some(value));
                        Ok(())
                    },
                    Err(_) => Err(SendError(key, value)),
                }
            }
        }

        /// Update the underlying hashmap with the given new key, value pair.
        /// Will return an error if the Mutex is currently locked.
        /// This method will never block.
        pub fn try_send(&self, key: K, value: V) -> Result<(), TrySendError<K, V>> {
            unsafe {
                let data_ptr = (*self.data.get()).get();
                *data_ptr = match (*data_ptr).downgrade().upgrade() {
                    Some(data) => data,
                    None => return Err(TrySendError::ChannelClosed(key, value)),
                };
                match (*data_ptr).try_lock() {
                    Ok(mut guard) => {
                        guard.insert(key, Some(value));
                        Ok(())
                    },
                    Err(err) => match err {
                        TryLockError::Poisoned(_) => Err(TrySendError::ChannelClosed(key, value)),
                        TryLockError::WouldBlock => Err(TrySendError::WouldBlock(key, value)),
                    }
                }
            }
        }

    }

    /// The receiver of the HashMap elements.
    pub struct Receiver<K: Send, V: Send> {
        data: UnsafeCell<Arc<Mutex<HashMap<K, Option<V>>>>>,
    }

    /// The Sender was dropped and the channel was closed.
    pub struct RecvError;

    /// An enumeration of the different possible try_recv errors.
    pub enum TryRecvError {
        /// The Senders were dropped and the channel was closed.
        ChannelClosed,
        /// A Sender has acquired the mutex and waiting would block the thread.
        WouldBlock,
    }

    impl<K: Send + Hash + Eq + Clone, V: Send> Receiver<K, V> {

        /// Receive the latest updates to the HashMap.
        /// May briefly block if a Sender is currently sending a key value pair.
        pub fn recv(&self) -> Result<Vec<(K, V)>, RecvError> {
            unsafe {
                let data_ptr = self.data.get();
                *data_ptr = match (*data_ptr).downgrade().upgrade() {
                    Some(data) => data,
                    None => return Err(RecvError),
                };
                match (*data_ptr).lock() {
                    Ok(mut guard) => {
                        Ok(guard.iter_mut().filter_map(|(key, value)| {
                            match value.is_some() {
                                true => Some((key.clone(), value.take().unwrap())),
                                false => None,
                            }
                        }).collect())
                    },
                    Err(_) => Err(RecvError),
                }
            }
        }

        /// Receive the latest updates to the HashMap.
        /// Will return an error if the Mutex is currently locked.
        /// This method will never block.
        pub fn try_recv(&self) -> Result<Vec<(K, V)>, TryRecvError> {
            unsafe {
                let data_ptr = self.data.get();
                *data_ptr = match (*data_ptr).downgrade().upgrade() {
                    Some(data) => data,
                    None => return Err(TryRecvError::ChannelClosed),
                };
                match (*data_ptr).try_lock() {
                    Ok(mut guard) => {
                        Ok(guard.iter_mut().filter_map(|(key, value)| {
                            match value.is_some() {
                                true => Some((key.clone(), value.take().unwrap())),
                                false => None,
                            }
                        }).collect())
                    },
                    Err(err) => match err {
                        TryLockError::Poisoned(_) => Err(TryRecvError::ChannelClosed),
                        TryLockError::WouldBlock => Err(TryRecvError::WouldBlock),
                    }
                }
            }
        }

    }

    /// Construct a Sender Receiver pair.
    pub fn channel<K: Send + Hash + Eq, V: Send>() -> (Sender<K, V>, Receiver<K, V>) {
        let arc = Arc::new(Mutex::new(HashMap::new()));
        let sender_arc = Arc::new(UnsafeCell::new(arc.clone()));
        (Sender { data: UnsafeCell::new(sender_arc), }, Receiver { data: UnsafeCell::new(arc), })
    }

}
