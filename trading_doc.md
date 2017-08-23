# Econ-Sim Trading

### Overview
This document goes over the theory behind the trading model in Econ-Sim. Then it goes into the techincals of how the current trading system works, limitation and future improvements to be made.

### Theory

In Econ-Sim, Trading is a core component of the project. The trading system in Econ-Sim is based off the law of supply and demand. Supply and demand works as such: As the supply of a good increases, the price of the good will decrease unless an equal increase in demand occurs. In the same regard, as the demand of a good increases, the price of the good will increase unless an equal increase in supply occcurs.

![alt text](/images/supply_and_demand_graph.gif "Supply and Demand")

In Econ-Sim, we have a group of villages that are producing goods (recources) each turn. Some will be able to better produce certain goods then other villages. For example, village 1 may more effectivly be able to produce food, while village 2 will be able to more effectivly produce stone. This creates, a situation where it is ideal for village 1 to sell food to village 2 and village 2 to sell stone to village 1. Therefore, each turn, villages will submit trade request on goods that they are willing to sell and buy. If the simulator cannot fulfill the requests (more demand then supply, or vise versa), the prices of goods will change until the prices find a new equilibrium price that can best satisfy the law of supply and demand.

### Techincals
The price of a good can have the following directions:
1. Equilibrium, the price of the good is best satisfying the law of supply and demand
2. Upward, the price of the good is moving upwards due to more demand than supply
3. Downward, the price of the good is moving downwards due to more supply than demand

Trading follows the following steps:
1. The Village Mind will submit trade requests on the goods it is willing to sell and what resources it wants to buy. This will be based off a variety of factors (current price of goods, supply, need etc.)
2. The Simulator will handle the trade requests in which three scenarios can occur: 
	1. The amount of buys and sells for a good are equal. All trade requests for that good are fulfilled and the price remains unchagned
	2. The desired amount of buys for a good are greater then the amount of sells. No trade request is fulfilled, and the price increases (more demand then supply). The price direction of the good is marked upwards. 
	3. The amount of a good being sold is greater then the amount of a good willing to be bought. No trade request is fulfilled and the price decreases (more supply then demand). The price direction of the good is marked as decreasing.
3. If any goods are not at equilibrium, the simulator will take in another round of trade requests. The village minds submit new trade requests based on the updated prices
4. Step 2 and 3 will be repeated until all goods hit an equilibrium price. However, step 2 gains additional complexity. If the price direction of a good was downwards and it now has more demand then supply at the new price, the simulator will fulfill the request to the best of its ability and consider the price at equilibrium. Vise versa if the price direction of a good was upwards.

### Limitations
We have yet to do actual integration tests to get results on the trading system. Also, it is highly unlikely we will ever have an "ideal" equilibrium price where all request to buy and sell are meet. Instead, the trading system will try to get as close as possible to an equilibrium price.

### Future Improvements
Currently we do a round robin when best trying to fulfill trade requests. In the future we can implement better systems for this. 
