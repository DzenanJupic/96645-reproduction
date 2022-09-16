/// https://github.com/rust-lang/rust/issues/96645#issue-1223189531
pub mod a {
    use std::collections::HashMap;

    pub struct A {
        module_state: RwLock<HashMap<WorkflowGroupId, HashMap<Fmid, Vec<u8>>>>,
    }

    #[derive(PartialEq, Eq, Hash)]
    pub struct WorkflowGroupId;

    #[derive(PartialEq, Eq, Hash)]
    pub struct Fmid;

    pub struct Semaphore;

    pub struct RwLock<T: ?Sized> {
        mr: u32,
        s: Semaphore,
        c: std::cell::UnsafeCell<T>,
    }

    pub struct RwLockReadGuard<'a, T: ?Sized> {
        s: &'a Semaphore,
        data: *const T,
        marker: std::marker::PhantomData<&'a T>,
    }

    impl<T: ?Sized> RwLock<T> {
        pub async fn read(&self) -> RwLockReadGuard<'_, T> { todo!() }
    }

    impl<'a, T: ?Sized> RwLockReadGuard<'a, T> {
        pub fn map<F, U: ?Sized>(this: Self, f: F) -> RwLockReadGuard<'a, U>
            where F: FnOnce(&T) -> &U { todo!() }
    }

    impl<'a, T: ?Sized> std::ops::Deref for RwLockReadGuard<'a, T> {
        type Target = T;
        fn deref(&self) -> &Self::Target { todo!() }
    }

    impl A {
        pub async fn get_module_state(
            &'_ self,
            workflow_id: &WorkflowGroupId,
            fmid: &Fmid,
        ) -> Option<RwLockReadGuard<'_, [u8]>> {
            let map_guard = self.module_state.read().await;

            let contains_state = map_guard
                .get(workflow_id)
                .map(|map| map.contains_key(fmid))
                .unwrap_or_default();

            if !contains_state {
                return None;
            }

            Some(RwLockReadGuard::map(map_guard, |map| {
                &**map.get(workflow_id).unwrap().get(fmid).unwrap()
            }))
        }
    }
}
