#[derive(Debug)]
pub struct ContractOne {
    pub input_data: String,
    pub output_data: Option<Result<String, String>>,
}

#[derive(Debug)]
pub struct ContractTwo {
    input_data: String,
    output_data: Option<Result<String, String>>,
}

fn handle_contract_one(mut contract: ContractOne) -> ContractOne {
    println!("{}", contract.input_data);
    contract.output_data = Some(Ok("Output Data".to_string()));
    contract
}

fn handle_contract_two(mut contract: ContractTwo) -> ContractTwo {
    println!("{}", contract.input_data);
    contract.output_data = Some(Ok("Output Data".to_string()));
    contract
}

#[macro_export]
macro_rules! register_contract_routes {
    (
        $handler_enum:ident,
        $fn_name:ident,
        $( $contract:ident => $handler_fn:path ),*) => {
            pub fn $fn_name(received_msg: $handler_enum) -> $handler_enum {
                match received_msg {
                    $(
                        $handler_enum::$contract(inner) => {
                            let executed_contract = $handler_fn(inner);
                            return $handler_enum::$contract(
                                executed_contract
                            )
                        }
                    )*
                }
            }
    };
}

#[derive(Debug, Clone)]
pub struct User {
    name: String,
    age: u32,
}

trait GetUsers {
    fn get_users() -> Vec<User>;
    fn get_user_by_name(name: &str) -> Option<User> {
        let users = Self::get_users();
        users.iter().find(|u| u.name == name).cloned()
    }
}

pub struct PostgresDB;

impl GetUsers for PostgresDB {
    fn get_users() -> Vec<User> {
        vec![User {
            name: String::from("CAL"),
            age: 30,
        }]
    }
}

#[derive(Debug)]
pub struct GetUserContract {
    pub name: String,
    pub user: Option<User>,
}

fn handle_get_user_by_name<T: GetUsers>(contract: GetUserContract) -> GetUserContract {
    let user = T::get_user_by_name(&contract.name);
    GetUserContract {
        name: contract.name,
        user: user,
    }
}

#[derive(Debug)]
pub enum ContractHandler {
    ContractOne(ContractOne),
    ContractTwo(ContractTwo),
    GetUserContract(GetUserContract),
}

register_contract_routes!(
    ContractHandler,
    handle_contract_xxx,
    ContractOne => handle_contract_one,
    ContractTwo => handle_contract_two,
    GetUserContract => handle_get_user_by_name::<PostgresDB>
);
