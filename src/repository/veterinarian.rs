use sqlx::error::Error;
use sqlx::postgres::PgPool;
use async_trait::async_trait;
use crate::repository::repository::Repository;
use crate::domain::veterinarian::Veterinarian;


pub struct VeterinarianRepository {
    pool: PgPool,
}

impl VeterinarianRepository {
    pub fn new(pool: PgPool) -> Self {
        VeterinarianRepository {
            pool: pool,
        }
    }
}

#[async_trait]
impl Repository<Veterinarian> for VeterinarianRepository {
    async fn save(&mut self, payload: Veterinarian) -> Result<(), Error> {
        // TODO: implementar usando sqlx
        Ok(())
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Veterinarian>, Error> {
        match sqlx::query_as!(Veterinarian,"SELECT id, name, \"inscricaoCRMV\" as inscricao_crmv FROM veterinarian WHERE id = $1", id)
        .fetch_optional(&self.pool)
        .await
        {
            Ok(veterinarian) => Ok(veterinarian),
            Err(err) => Err(Error::from(err))
        }
    }

    async fn remove(&mut self, id: i32) -> Result<(), Error> {
        let _query_result = sqlx::query!("DELETE FROM veterinarian WHERE id = $1", id)
        .execute(&self.pool)
        .await
        .map_err(Error::from)?;

        Ok(())
    }

    async fn patch(&mut self, id: i32, payload: Veterinarian) -> Result<(), Error> {
        // TODO: implementar usando sqlx
        Ok(())
    }
}
