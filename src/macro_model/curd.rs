use crate::*;

#[async_trait]
pub trait CURD
where
    Self: Sized,
{
    async fn create(&self) -> bool;
    async fn delete(&id: Thing) -> bool;
    async fn get_one(&id: Thing) -> Vec<Self>;
}

#[macro_export]
macro_rules! curd {
    ($item:ident) => {
        paste! {
            #[async_trait]
            impl CURD for $item {
                    #[doc = "Create a new `" $item "` object."]
                    async fn create(&self) -> bool {
                        format!("create {} CONTENT {}",stringify!($item),self).execute_silent().await
                        // true
                    }

                    #[doc = "Delete a `" $item "` object by id."]
                    async fn delete(id: Thing) -> bool {
                        format!("delete {}",id).execute_silent().await
                    }
                    #[doc = "get a `" $item "` object by id."]
                    async fn get_one(id: Thing) -> Vec<Self> {
                        format!("select * from {}",id).execute_one::<Self>().await
                    }
            }
        }
    };
}
#[cfg(test)]
curd!(App);
