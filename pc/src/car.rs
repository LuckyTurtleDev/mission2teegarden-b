
fn getRelativeXY(car_action: Option<CarAction>, orientation: Orientation) ->(i8, u8, Orientation){
    match car_action {
        None => (0, 0, orientation),
        TurnLeft => {
            match orientation {
                North => (-1, -1, West),
                South => (1, 1, East),
                West => (-1, 1, South),
                East => (1, -1, North)
            }
        },
        TurnRight => {
            match orientation {
                North => (-1, 1, East),
                South => (-1, 1, West),
                West => (-1, -1, North),
                East => (1, 1, South)
            }
        },
        DriveForward => {
            match orientation {
                North => (0, -1, North),
                South => (0, South),
                West => (-1, West),
                East => (1, East)
            }
        }
    }
}

fn car_movement(map: &Map, player: &Player, intended_moves: CardStatus) -> CardStatus {
    let start_coordinates = player.start;
    let mut player_XY = player.start;
    let mut cards = intended_moves;
    cards.iter_mut().for_each(|element| {

    });
}