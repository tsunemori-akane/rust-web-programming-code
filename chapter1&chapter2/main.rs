mod test_marco;
mod test_struct_state;
mod test_trait;
use test_marco::*;
use test_struct_state::*;
use test_trait::*;
fn main() {
    // let admin = AdminUser {
    //     username: String::from("admin"),
    //     password: String::from("123456"),
    // };
    // let user = User {
    //     username: String::from("CAL"),
    //     password: String::from("123456"),
    // };
    // create(&admin);
    // edit(&admin);
    // delete(&admin);
    // edit(&user);
    // cache(&admin);

    // let contract_one = ContractOne {
    //     input_data: "Contract One".to_string(),
    //     output_data: None,
    // };
    // let outcome = handle_contract_xxx(ContractHandler::ContractOne(contract_one));
    // println!("{:?}", outcome);

    let get_user_contract = GetUserContract {
        name: "CAL".to_string(),
        user: None,
    };
    let outcome = handle_contract_xxx(ContractHandler::GetUserContract(get_user_contract));
    println!("{:?}", outcome);

    let tx = Transaction::<InProgress>::new(1);
    let tx = tx.commit();
}
