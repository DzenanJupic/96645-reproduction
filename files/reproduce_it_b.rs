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
