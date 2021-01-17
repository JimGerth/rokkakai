use rokkakari::cell::*;

fn main() {
    println!("Constraint::Equal(2) = {:?}", bincode::serialize(&Constraint::Equal(2)).unwrap());
    println!("WallState::Unconstrained = {:?}", bincode::serialize(&WallState::Unconstrained).unwrap());
    println!("WallState::Constrained(Constraint::Equal(2)) = {:?}", bincode::serialize(&WallState::Constrained(Constraint::Equal(2))).unwrap());
    println!("{:?}", bincode::serialize(&Cell::Wall(WallState::Unconstrained)));

    let wall_cell = Cell::Wall(WallState::Constrained(Constraint::Equal(2)));
    let encoded_wall_cell: Vec<u8> = bincode::serialize(&wall_cell).unwrap();
    let decoded_wall_cell = bincode::deserialize(&encoded_wall_cell[..]).unwrap();
    println!("{:?} <{:?}>", decoded_wall_cell, encoded_wall_cell);
    assert_eq!(wall_cell, decoded_wall_cell);

    let free_cell = Cell::Free(FreeState { marking: Some(Marking::Lamp), lit_from_a: false, lit_from_b: false, lit_from_c: false, lit_from_d: true, lit_from_e: false, lit_from_f: true } );
    let encoded_free_cell: Vec<u8> = bincode::serialize(&free_cell).unwrap();
    let decoded_free_cell = bincode::deserialize(&encoded_free_cell[..]).unwrap();
    println!("{:?} <{:?}>", decoded_free_cell, encoded_free_cell);
    assert_eq!(free_cell, decoded_free_cell);
}
