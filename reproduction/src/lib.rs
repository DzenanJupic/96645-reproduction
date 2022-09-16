#![allow(unused)]

pub use tokio;
pub use sqlx;
pub use futures;
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

// Some comment

// Some comment

// Some comment
/// https://github.com/rust-lang/rust/issues/96645#issuecomment-1153740085
pub mod b {
    macro_rules! query_as {
        ($($tt:tt)*) => { Query::<SomeA> { marker: std::marker::PhantomData } };
    }
    macro_rules! query {
        ($($tt:tt)*) => { Query::<SomeA> { marker: std::marker::PhantomData } };
    }

    #[derive(Debug)]
    pub struct Grid {
        id: i64,
        row: f32,
    }
    pub struct Target {
        id: i64,
        grid_id: Option<i64>,
    }
    pub struct Transaction<'a, T> {
        marker: std::marker::PhantomData<&'a T>,
    }
    pub struct Postgres;
    pub struct Error;
    pub struct Query<'q, A> {
        marker: std::marker::PhantomData<&'q A>,
    }
    pub struct SomeA;

    impl<'q, A> Query<'q, A> {
        pub async fn fetch_one<'e, 'c: 'e, E>(self, executor: E) -> Result<Target, Error>
            where 'q: 'e, A: 'e, E: 'e { todo!() }

        pub async fn execute<'e, 'c: 'e, E>(self, executor: E) -> Result<(), Error>
            where 'q: 'e, A: 'e, E: 'c { todo!() }
    }

    /// Just for namespacing
    pub struct GridSql;

    impl GridSql {
        pub async fn add_empty<'t>(
            target: &Target,
            mut tx: Transaction<'t, Postgres>,
        ) -> Result<Transaction<'t, Postgres>, Error> {
            let grid_id = query_as!(
                Grid,
                r#"
                INSERT INTO grid (
                   row
                ) VALUES($1)
                RETURNING
                    id, ...
                "#,
                0.0,
            )
                .fetch_one(&mut tx)
                .await?
                .id;
            query!(
                "UPDATE target SET grid_id = $1 WHERE id = $2",
                grid_id,
                target.id,
            )
                .execute(&mut tx)
                .await?;
            Ok(tx)
        }
    }
}

// Some comment
/// https://github.com/rust-lang/rust/issues/96645#issuecomment-1153902305
mod c {
    pub struct C;
    pub struct RecKey;
    pub struct RecSeries;
    pub struct Error;

    impl C {
        pub async fn rs(&mut self, key: &RecKey) -> Result<&RecSeries, Error> { todo!() }
    }
}

// Some comment

// Some comment

// Some comment

// Some comment

// Some comment

// Some comment

// Some comment
