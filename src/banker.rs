pub struct Banker {
    pub pot: u32,
    pub money: Vec<u32>
}

impl Banker {
    pub fn new(players: u32, starting_money: u32) -> Self{
        let mut vec = Vec::with_capacity(players as usize);
        vec.resize(players as usize, starting_money);
        Self {
            pot: 0,
            money: vec
        }
    }

    pub fn bet(&mut self, player: u32, amt: u32) {
        self.money[player as usize] -= amt;
        self.pot += amt;
    }

    pub fn win(&mut self, player: u32, amt: u32) {
        self.money[player as usize] += self.pot;
        self.pot = 0;
    }
}