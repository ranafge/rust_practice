use rust_practice::Inventory;

fn main() {
    let mut inventory = Inventory::new(); // 3
    inventory.go_to_instock();
    inventory.sold();
    inventory.sold();
    inventory.sold();
    // inventory.go_to_outofstock();
    inventory.go_to_instock();
    inventory.add_item(2);
    inventory.go_to_outofstock();
    inventory.go_to_instock();
    inventory.go_to_discontinued();

 
}





//     // println!("{}", *r1);

// }
   

    
    

