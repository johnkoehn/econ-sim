pub mod village_manager;
pub mod simulation_settings;

use self::village_manager::*;
use village::*;
use village::resource::*;
use village_mind::*;
use village_mind::trade_request::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PriceDirection {
    Upward,
    Downward,
    Equilibrium,
}

pub struct Simulation {
    village_managers: Vec<VillageManager>,
    pub prices: HashMap<ResourceType, u32>,
    pub price_directions: HashMap<ResourceType, PriceDirection>,
}

impl Simulation {
    pub fn new() -> Simulation {

        let mut simulation = Simulation {
            village_managers: vec!(),
            prices: HashMap::new(),
            price_directions: HashMap::new(),
        };

        //for each resource (other than gold), put the starting price at one gold
        for resource_type in ResourceType::iterator() {
            if *resource_type != ResourceType::Gold {
                simulation.prices.insert(*resource_type, 5);
                simulation.price_directions.insert(*resource_type, PriceDirection::Equilibrium);
            }
        }
        simulation
    }

    pub fn add_village(&mut self, village: Village) {
        let villageRC = Rc::new(RefCell::new(village));
        let village_manager = VillageManager {
            village: villageRC.clone(),
            village_mind: VillageMind::new(villageRC.clone()),
        };

        self.village_managers.push(village_manager);
    }

    pub fn village_managers(&self) -> &Vec<VillageManager> {
        &self.village_managers
    }

    pub fn simulate(&mut self) {
        // update the villages and village minds with the new information
        for vm in self.village_managers.iter_mut() {
            vm.village.borrow_mut().simulate();
            vm.village_mind.manage_village();
        }

        // do the trading phase until no more trade request are given
        let mut trading = true;
        let mut trade_requests : Vec<TradeRequest> = Vec::new();

        while trading {
            trade_requests.clear();

            for mut vm in self.village_managers.iter_mut() {
                trade_requests.append(&mut vm.village_mind.trade());
            }

            if trade_requests.len() == 0 {
                trading = false;
            }
        }

        // update village minds
        for vm in self.village_managers.iter_mut() {
            vm.village_mind.manage_village();
        }
    }

    pub fn handle_trades(&mut self, trade_requests: &mut Vec<TradeRequest>) {
        // get the trade request for each resource type, excluding gold
        for resource_type in ResourceType::iterator().filter(|r| **r != ResourceType::Gold) {
            // get the total number of buys and sells requested for a resource at the current price
            let mut buys = 0;
            let mut sells = 0;

            for trade_request in trade_requests.iter_mut().filter(|t| t.resource_type == *resource_type) {
                if trade_request.trade_type == TradeType::Buy {
                    buys += trade_request.request_amount;
                } else {
                    sells += trade_request.request_amount;
                }
            }

            // if number of buys and sells are equal consider the price at equilibrium and fulfill the trade requests
            if buys == sells {
                for trade_request in trade_requests.iter_mut().filter(|t| t.resource_type == *resource_type) {
                    trade_request.fulfilled_amount = trade_request.request_amount;
                }
                self.price_directions.insert(*resource_type, PriceDirection::Equilibrium);
            }
            else if buys > sells {
                let price_direction = *self.price_directions.get(resource_type).unwrap();

                // equilibrium or upward: price will increase
                if price_direction == PriceDirection::Equilibrium || price_direction == PriceDirection::Upward {
                    self.price_directions.insert(*resource_type, PriceDirection::Upward);
                    let price = self.prices.get_mut(resource_type).unwrap();
                    *price += 1;
                }
                // downward: an acceptable equilibrium price was found
                else {
                    // fulfill all sell requests
                    for trade_request in trade_requests.iter_mut().filter(|t| t.resource_type == *resource_type && t.trade_type == TradeType::Sell) {
                        trade_request.fulfilled_amount = trade_request.request_amount;
                    }
                    
                    let mut buy_requests: Vec<&mut TradeRequest> = trade_requests.iter_mut()
                        .filter(|t| t.resource_type == *resource_type && t.trade_type == TradeType::Buy)
                        .collect();

                    // evenly distribute the sells amongst buyers
                    while sells > 0 {
                        for buy_request in buy_requests.iter_mut() {
                            buy_request.fulfilled_amount += 1;
                            sells -= 1;
                            if sells <= 0 { break; }
                        }
                    }

                    self.price_directions.insert(*resource_type, PriceDirection::Equilibrium);
                }
            }
            // more sellers than buyers
            else {
                let price_direction = *self.price_directions.get(resource_type).unwrap();
                let price = self.prices.get_mut(resource_type).unwrap();

                // equilibrium or downward: price will decrease
                if (price_direction == PriceDirection::Equilibrium || price_direction == PriceDirection::Downward) && *price > 1 {
                    self.price_directions.insert(*resource_type, PriceDirection::Downward);
                    *price -= 1;
                }
                // upward: an acceptable equilibrium price was found
                else {
                    // fulfill all buy requests
                    for buy_request in trade_requests.iter_mut().filter(|t| t.resource_type == *resource_type && t.trade_type == TradeType::Buy) {
                        buy_request.fulfilled_amount = buy_request.request_amount;
                    }

                    let mut sell_requests : Vec<&mut TradeRequest> = trade_requests.iter_mut()
                        .filter(|t| t.resource_type == *resource_type && t.trade_type == TradeType::Sell)
                        .collect();

                    // evenly distribute the buys amongst sellers
                    while buys > 0 {
                        for sell_request in sell_requests.iter_mut() {
                            sell_request.fulfilled_amount += 1;
                            buys -= 1;
                            if buys <= 0 {break;}
                        }
                    }

                    self.price_directions.insert(*resource_type, PriceDirection::Equilibrium);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use village::*;
    use village_mind::*;
    use simulation::*;
    use village::resource::*;
    use village_mind::trade_request::*;

    fn default_village() -> Village {
        Village::new(|w: &worker::Worker| false)
    }

    #[test]
    fn add_village() {
        let mut simulation = Simulation::new();
        simulation.add_village(default_village());

        assert_eq!(1, simulation.village_managers().len());
    }

    #[test]
    fn initial_resource_prices() {
        let mut simulation = Simulation::new();

        assert_eq!(5, *simulation.prices.get(&ResourceType::Food).unwrap());
    }

    #[test]
    fn initial_price_directions() {
        let mut simulation = Simulation::new();

        assert_eq!(PriceDirection::Equilibrium, *simulation.price_directions.get(&ResourceType::Food).unwrap());
    }

    #[test]
    fn handle_trades_basic_sell_and_buy() {
        let mut simulation = Simulation::new();
        let mut trade_requests : Vec<TradeRequest> = Vec::new();
        trade_requests.push(TradeRequest::new(TradeType::Buy, 1, ResourceType::Food));
        trade_requests.push(TradeRequest::new(TradeType::Sell, 1, ResourceType::Food));
        simulation.handle_trades(&mut trade_requests);

        assert_eq!(1, trade_requests.get(0).unwrap().fulfilled_amount);
        assert_eq!(1, trade_requests.get(1).unwrap().fulfilled_amount);
        assert_eq!(PriceDirection::Equilibrium, *simulation.price_directions.get(&ResourceType::Food).unwrap());
    }

    #[test]
    fn handle_trades_upward_price() {
        let mut simulation = Simulation::new();
        let mut trade_requests : Vec<TradeRequest> = Vec::new();
        trade_requests.push(TradeRequest::new(TradeType::Buy, 1, ResourceType::Food));
        simulation.handle_trades(&mut trade_requests);

        assert_eq!(0, trade_requests.get(0).unwrap().fulfilled_amount);
        assert_eq!(PriceDirection::Upward, *simulation.price_directions.get(&ResourceType::Food).unwrap());
        assert_eq!(6, *simulation.prices.get(&ResourceType::Food).unwrap());
    }

    #[test]
    fn handle_trades_downward_price() {
        let mut simulation = Simulation::new();
        let mut trade_requests : Vec<TradeRequest> = Vec::new();
        trade_requests.push(TradeRequest::new(TradeType::Sell, 1, ResourceType::Food));
        simulation.handle_trades(&mut trade_requests);

        assert_eq!(0, trade_requests.get(0).unwrap().fulfilled_amount);
        assert_eq!(PriceDirection::Downward, *simulation.price_directions.get(&ResourceType::Food).unwrap());
        assert_eq!(4, *simulation.prices.get(&ResourceType::Food).unwrap());
    }

    #[test]
    fn handle_trades_multiple_rounds_downward() {
        let mut simulation = Simulation::new();
        let mut trade_requests : Vec<TradeRequest> = Vec::new();
        trade_requests.push(TradeRequest::new(TradeType::Sell, 1, ResourceType::Food));
        simulation.handle_trades(&mut trade_requests);
        simulation.handle_trades(&mut trade_requests);
        simulation.handle_trades(&mut trade_requests);

        trade_requests.push(TradeRequest::new(TradeType::Buy, 5, ResourceType::Food));
        simulation.handle_trades(&mut trade_requests);

        assert_eq!(1, trade_requests.get(0).unwrap().fulfilled_amount);
        assert_eq!(1, trade_requests.get(1).unwrap().fulfilled_amount);
        assert_eq!(PriceDirection::Equilibrium, *simulation.price_directions.get(&ResourceType::Food).unwrap());
        assert_eq!(2, *simulation.prices.get(&ResourceType::Food).unwrap());
    }

    #[test]
    fn handle_trades_multiple_rounds_upward() {
        let mut simulation = Simulation::new();
        let mut trade_requests : Vec<TradeRequest> = Vec::new();
        trade_requests.push(TradeRequest::new(TradeType::Buy, 1, ResourceType::Food));
        simulation.handle_trades(&mut trade_requests);
        simulation.handle_trades(&mut trade_requests);
        simulation.handle_trades(&mut trade_requests);

        trade_requests.push(TradeRequest::new(TradeType::Sell, 5, ResourceType::Food));
        simulation.handle_trades(&mut trade_requests);

        assert_eq!(1, trade_requests.get(0).unwrap().fulfilled_amount);
        assert_eq!(1, trade_requests.get(1).unwrap().fulfilled_amount);
        assert_eq!(PriceDirection::Equilibrium, *simulation.price_directions.get(&ResourceType::Food).unwrap());
        assert_eq!(8, *simulation.prices.get(&ResourceType::Food).unwrap());
    }

    #[test]
    fn handle_trades_price_minimum() {
        let mut simulation = Simulation::new();
        let mut trade_requests : Vec<TradeRequest> = Vec::new();
        trade_requests.push(TradeRequest::new(TradeType::Sell, 2, ResourceType::Food));
        simulation.handle_trades(&mut trade_requests);
        simulation.handle_trades(&mut trade_requests);
        simulation.handle_trades(&mut trade_requests);
        simulation.handle_trades(&mut trade_requests);
        trade_requests.push(TradeRequest::new(TradeType::Buy, 1, ResourceType::Food));
        simulation.handle_trades(&mut trade_requests);


        assert_eq!(1, trade_requests.get(0).unwrap().fulfilled_amount);
        assert_eq!(1, trade_requests.get(1).unwrap().fulfilled_amount);
        assert_eq!(PriceDirection::Equilibrium, *simulation.price_directions.get(&ResourceType::Food).unwrap());
        assert_eq!(1, *simulation.prices.get(&ResourceType::Food).unwrap());
    }

    #[test]
    fn handle_trades_no_requests() {
        let mut simulation = Simulation::new();
        let mut trade_requests : Vec<TradeRequest> = Vec::new();
        trade_requests.push(TradeRequest::new(TradeType::Sell, 2, ResourceType::Food));
        simulation.handle_trades(&mut trade_requests);
        simulation.handle_trades(&mut trade_requests);
        trade_requests.clear();
        simulation.handle_trades(&mut trade_requests);

        assert_eq!(PriceDirection::Equilibrium, *simulation.price_directions.get(&ResourceType::Food).unwrap());
    }
}