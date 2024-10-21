use metashrew_support::byte_view::ByteView;
use crate::{get, set};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct IndexPointer(Arc<Vec<u8>>);

#[allow(dead_code)]
pub trait KeyValuePointer {
    fn wrap(word: &Vec<u8>) -> Self;
    fn unwrap(&self) -> Arc<Vec<u8>>;
    fn set(&mut self, v: Arc<Vec<u8>>);
    fn get(&self) -> Arc<Vec<u8>>;
    fn inherits(&mut self, from: &Self);
    fn select(&self, word: &Vec<u8>) -> Self
    where
        Self: Sized,
    {
        let mut key = (*self.unwrap()).clone();
        key.extend(word);
        let mut ptr = Self::wrap(&key);
        ptr.inherits(self);
        ptr
    }
    fn from_keyword(word: &str) -> Self
    where
        Self: Sized,
    {
        Self::wrap(&word.as_bytes().to_vec())
    }
    fn keyword(&self, word: &str) -> Self
    where
        Self: Sized,
    {
        let mut key = (*self.unwrap()).clone();
        key.extend(word.to_string().into_bytes());
        let mut ptr = Self::wrap(&key);
        ptr.inherits(self);
        ptr
    }

    fn set_value<T: ByteView>(&mut self, v: T) {
        self.set(Arc::new(v.to_bytes()));
    }

    fn get_value<T: ByteView>(&self) -> T {
        let cloned = self.get().as_ref().clone();
        if cloned.is_empty() {
            T::zero()
        } else {
            T::from_bytes(cloned)
        }
    }

    fn select_value<T: ByteView>(&self, key: T) -> Self
    where
        Self: Sized,
    {
        self.select(key.to_bytes().as_ref())
    }
    fn length_key(&self) -> Self
    where
        Self: Sized,
    {
        self.keyword(&"/length".to_string())
    }
    fn length(&self) -> u32
    where
        Self: Sized,
    {
        self.length_key().get_value::<u32>()
    }
    fn select_index(&self, index: u32) -> Self
    where
        Self: Sized,
    {
        self.keyword(&format!("/{}", index))
    }

    fn get_list(&self) -> Vec<Arc<Vec<u8>>>
    where
        Self: Sized,
    {
        let mut result: Vec<Arc<Vec<u8>>> = vec![];
        for i in 0..self.length() {
            result.push(self.select_index(i as u32).get().clone());
        }
        result
    }
    fn get_list_values<T: ByteView>(&self) -> Vec<T>
    where
        Self: Sized,
    {
        let mut result: Vec<T> = vec![];
        for i in 0..self.length() {
            result.push(self.select_index(i as u32).get_value());
        }
        result
    }
    fn nullify(&mut self) {
        self.set(Arc::from(vec![0]))
    }
    fn set_or_nullify(&mut self, v: Arc<Vec<u8>>) {
        let val = Arc::try_unwrap(v).unwrap();
        if <usize>::from_bytes(val.clone()) == 0 {
            self.nullify();
        } else {
            self.set(Arc::from(val));
        }
    }

    fn pop(&self) -> Arc<Vec<u8>>
    where
        Self: Sized,
    {
        let mut length_key = self.length_key();
        let length = length_key.get_value::<u32>();

        if length == 0 {
            return Arc::new(Vec::new()); // Return empty Vec if there are no elements
        }

        let new_length = length - 1;
        length_key.set_value::<u32>(new_length); // Update the length
        self.select_index(new_length).get() // Return the value at the new length
    }

    fn pop_value<T: ByteView>(&self) -> T
    where
        Self: Sized,
    {
        let mut length_key = self.length_key();
        let length = length_key.get_value::<u32>();

        if length == 0 {
            return T::from_bytes(Vec::new()); // Return a default value if there are no elements
        }

        let new_length = length - 1;
        length_key.set_value::<u32>(new_length); // Update the length
        self.select_index(new_length).get_value::<T>() // Return the value at the new length
    }

    fn append(&self, v: Arc<Vec<u8>>)
    where
        Self: Sized,
    {
        let mut new_index = self.extend();
        new_index.set(v);
    }

    fn append_value<T: ByteView>(&self, v: T)
    where
        Self: Sized,
    {
        let mut new_index = self.extend();
        new_index.set_value(v);
    }

    fn extend(&self) -> Self
    where
        Self: Sized,
    {
        let mut length_key = self.length_key();
        let length = length_key.get_value::<u32>();
        length_key.set_value::<u32>(length + 1);
        self.select_index(length)
    }
    fn prefix(&self, keyword: &str) -> Self
    where
        Self: Sized,
    {
        let mut val = keyword.to_string().into_bytes();
        val.extend((*self.unwrap()).clone());
        let mut ptr = Self::wrap(&val);
        ptr.inherits(self);
        ptr
    }
}

#[allow(dead_code)]
impl KeyValuePointer for IndexPointer {
    fn wrap(word: &Vec<u8>) -> IndexPointer {
        IndexPointer(Arc::<Vec<u8>>::new(word.clone()))
    }
    fn unwrap(&self) -> Arc<Vec<u8>> {
        self.0.clone()
    }
    fn inherits(&mut self, _v: &Self) {}
    fn set(&mut self, v: Arc<Vec<u8>>) {
        set(self.unwrap(), v)
    }
    fn get(&self) -> Arc<Vec<u8>> {
        get(self.unwrap())
    }
}

#[derive(Clone, Default, Debug)]
pub struct IndexCheckpoint(pub HashMap<Arc<Vec<u8>>, Arc<Vec<u8>>>);

impl IndexCheckpoint {
    fn pipe_to(&self, target: &mut IndexCheckpoint) {
        self.0.iter().for_each(|(k, v)| {
            target.0.insert(k.clone(), v.clone());
        });
    }
}

#[derive(Clone, Debug)]
pub struct IndexCheckpointStack(pub Arc<Mutex<Vec<IndexCheckpoint>>>);

impl Default for IndexCheckpointStack {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(vec![IndexCheckpoint::default()])))
    }
}

#[derive(Clone, Debug)]
pub struct AtomicPointer {
    pointer: IndexPointer,
    store: IndexCheckpointStack,
}

impl KeyValuePointer for AtomicPointer {
    fn wrap(word: &Vec<u8>) -> Self {
        AtomicPointer {
            pointer: IndexPointer::wrap(word),
            store: IndexCheckpointStack::default(),
        }
    }
    fn unwrap(&self) -> Arc<Vec<u8>> {
        self.pointer.unwrap()
    }
    fn inherits(&mut self, from: &Self) {
        self.store = from.store.clone()
    }
    fn set(&mut self, v: Arc<Vec<u8>>) {
        self.store
            .0
            .lock()
            .unwrap()
            .last_mut()
            .unwrap()
            .0
            .insert(self.unwrap(), v.clone());
    }
    fn get(&self) -> Arc<Vec<u8>> {
        let unwrapped = self.unwrap();
        match self
            .store
            .0
            .lock()
            .unwrap()
            .iter()
            .rev()
            .find(|map| map.0.contains_key(&unwrapped))
        {
            Some(map) => map.0.get(&unwrapped).unwrap().clone(),
            None => self.pointer.get(),
        }
    }
}

impl Default for AtomicPointer {
    fn default() -> Self {
        AtomicPointer {
            pointer: IndexPointer::wrap(&Vec::<u8>::new()),
            store: IndexCheckpointStack::default(),
        }
    }
}

impl AtomicPointer {
    pub fn checkpoint(&mut self) {
        self.store
            .0
            .lock()
            .unwrap()
            .push(IndexCheckpoint::default());
    }
    pub fn commit(&mut self) {
        let checkpoints = &mut self.store.0.lock().unwrap();
        if checkpoints.len() > 1 {
            checkpoints
                .pop()
                .unwrap()
                .pipe_to(checkpoints.last_mut().unwrap());
        } else if checkpoints.len() == 1 {
            checkpoints.last().unwrap().0.iter().for_each(|(k, v)| {
                set(k.clone(), v.clone());
            });
        } else {
            panic!("commit() called without checkpoints in memory");
        }
    }
    pub fn rollback(&mut self) {
        self.store.0.lock().unwrap().pop();
    }
    pub fn derive(&self, pointer: &IndexPointer) -> Self {
      AtomicPointer {
        store: self.store.clone(),
        pointer: pointer.clone()
      }
    }
}
