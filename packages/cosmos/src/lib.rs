pub mod error;
pub mod modules;
pub use error::Error;

pub mod ext;

#[cfg(feature = "cosmos")]
#[cfg(test)]
mod tests {
    use crate::modules::cosmos::auth::{
        module::v1::ModuleAccountPermission,
        v1beta1::{BaseAccount, QueryAccountInfoResponse},
    };
    use prost_types::Any;

    #[test]
    fn test_try_decode_valid() {
        // we wrap anything inside ANy
        let foo_msg: ModuleAccountPermission = ModuleAccountPermission {
            account: "1".to_string(),
            permissions: vec![],
        };

        let mut base_account: BaseAccount = BaseAccount::default();

        let any = Any::from_msg::<ModuleAccountPermission>(&foo_msg).unwrap();
        base_account.address = "test1".to_string();
        base_account.pub_key = Some(any);

        let response = QueryAccountInfoResponse {
            info: Some(base_account.clone()),
        };

        let account: BaseAccount = response.info.expect("Cant find account");
        assert_eq!(account, base_account);
    }
}
