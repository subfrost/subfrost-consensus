#[derive(Default)]
pub struct AlkaneGlobalState {
    pub store: Vec<AlkaneCheckpoint>,
    pub context: AlkaneMessageContext,
}

impl AlkaneGlobalState {
    pub fn from(context: &AlkaneMessageContext) -> AlkaneGlobalState {
        AlkaneGlobalState {
            store: Vec::<AlkaneCheckpoint>::new(),
            context,
        }
    }
}

#[derive(Default, Clone)]
pub struct AlkaneCheckpoint(pub HashMap<AlkaneId, AlkaneState>);

#[derive(Default, Clone)]
pub struct AlkaneState {
    balances: HashMap<AlkaneId, u128>,
    storage: HashMap<Vec<u8>, Vec<u8>>,
}

impl AlkaneCheckpoint {
    pub fn pipe_to(target: &mut AlkaneContext) {
        self.0.iter_mut().for_each(|k, v| {
            let item = target.0.get(k.clone());
            if item.is_none() {
                target.0.set(k.clone(), v.clone());
            } else {
                v.pipe_to(target);
            }
        });
    }
    pub fn flush(ptr: &AtomicPointer) {
        self.0.iter().for_each(|k, v| {
            v.flush(k, ptr);
        });
    }
    pub fn lazy_get(&mut self, who: &AlkaneId) -> &AlkaneState {
        if !self.contains_key(who) {
            self.set(who, AlkaneState::default());
        }
        &self.get(who)
    }
    pub fn lazy_get_mut(&mut self, who: &AlkaneId) -> &mut AlkaneState {
        if !self.contains_key(who) {
            self.set(who, AlkaneState::default());
        }
        &mut self.get(who)
    }
}

impl AlkaneState {
    fn _index_balances(&self, who: AlkaneId, ptr: AtomicPointer) {
        let serialized = who.serialize();
        let balance_pointer = ptr.keyword("balances/");
        self.balances.iter().for_each(|k, v| {
            balance_pointer
                .select(serialized)
                .keyword("/")
                .select(k.serialize())
                .set(v.to_le_bytes());
        });
    }
    fn _index_storage(&self, who: AlkaneId, ptr: AtomicPointer) {
        let serialized = who.serialize();
        let storage_pointer = ptr.keyword("storage/");
        self.storage.iter().for_each(|k, v| {
            storage_pointer
                .select(serialized)
                .keyword("/")
                .select(k)
                .set(v);
        });
    }
    fn _pipe_balances_to(&self, target: &mut AlkaneState) {
        self.balances.iter().for_each(|k, v| {
            target.balances.set(k.clone(), v.clone());
        });
    }
    fn _pipe_storage_to(&self, target: &mut AlkaneState) {
        self.storage.iter().for_each(|k, v| {
            target.storage.set(k.clone(), v.clone());
        });
    }
    fn flush(&self, who: AlkaneId, ptr: AtomicPointer) {
        self._index_balances(who, ptr);
        self._flush_storage(who, ptr);
    }
    fn pipe_to(&self, target: &mut AlkaneState) {
        self._pipe_balances_to(target);
        self._pipe_storage_to(target);
    }
}

impl AlkaneGlobalState {
    pub fn checkpoint(&mut self) {
        self.store.push(AlkaneCheckpoint::default());
    }
    pub fn current(&mut self) -> Option<&mut AlkaneCheckpoint> {
        self.store.last()
    }
    pub fn balance(&self, who: &AlkaneId, what: &AlkaneId) -> Option<u128> {
        Some(
            (self.store.iter().rev().find(|v| {
                if v.contains_key(who) && v.get(who).balances.contains_key(what) {
                    Some(v)
                } else {
                    None
                }
            })?)
            .get(who)?
            .balances
            .get(what)?,
        )
    }
    pub fn lookup(&self, who: &AlkaneId, what: &Vec<u8>) -> Option<Vec<u8>> {
        Some(
            (self.store.iter().rev().find(|v| {
                if v.contains_key(who) && v.get(who).storage.contains_key(what) {
                    Some(v)
                } else {
                    None
                }
            })?)
            .get(who)
            .storage
            .get(what)?,
        )
    }
    pub fn rollback(&mut self) {
        self.store.pop();
    }
    pub fn commit(&mut self) -> bool {
        if self.store.len() > 1 {
            self.store.pop().pipe_to(self.store.last());
            return true;
        } else if self.store.len() == 1 {
            self.store.pop().flush(self.context.runtime);
            return true;
        } else {
            return false;
        }
    }
    pub fn set_balance(&mut self, who: &AlkaneId, what: &AlkaneId, balance: u128) {
        self.store
            .last()
            .lazy_get_mut(who)
            .balances
            .set(what, balance);
    }
    pub fn set_storage(&mut self, who: &AlkaneId, what: &Vec<u8>, value: &Vec<u8>) {
        self.store
            .last()
            .lazy_get_mut(who)
            .storage
            .set(what.clone(), value.clone());
    }
}
