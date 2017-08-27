# Econ-Sim Trading

### Overview
This document goes over the theory behind the trading model. Then it goes into the techincals of how the current trading system works, limitations and future improvements to be made.

### Theory
Trading is based on the law of supply and demand. As the supply of a resource increases, the price will decrease unless an equal increase in demand occurs. In the same regard, as the demand of a resource increases, the price will increase unless an equal increase in supply occcurs.

![alt text](/images/supply_and_demand_graph.gif "Supply and Demand")

Each turn, villages produce resources at a rate determined by several factors (e.g. assigned workers, difficulty). Some villages may not even have access to certain resources.

These differences in resource production rates create a reason for trade between villages. If a village has a surplus of a resource, it will likely submit a trade request offering to sell that resource. If a village has a scarcity of a resource, it will likely submit a trade request offering to buy that resource. If the simulator cannot fulfill the requests (more demand then supply, or vise versa), the prices of resources will change until the prices find a new equilibrium price that can best satisfy the law of supply and demand.

### Techincals
The price of a resource can have the following directions:
1. Equilibrium: the price of the resource is best satisfying the law of supply and demand
2. Upward: the price of the resource is moving upwards due to more demand than supply
3. Downward: the price of the resource is moving downwards due to more supply than demand

Trading follows these steps:
1. Village mind submits trade requests on the resources it is willing to buy and sell. These requests are determined from various factors (current price of resources, supply, need).
2. The simulator handles the trade requests. Three scenarios can occur: 
	1. The amount of buys and sells for a resource are equal. All trade requests for that resource are fulfilled and the price remains unchagned.
	2. The desired amount of buys for a resource are greater then the amount of sells. No trade request is fulfilled, and the price increases (more demand than supply).
	3. The amount of a resource being sold is greater than the amount of a resource willing to be bought. No trade request is fulfilled and the price decreases (more supply then demand).
3. If any resources are not at equilibrium, the simulator will take in another round of trade requests. The village minds submit new trade requests based on the updated prices.
4. Step 2 and 3 will be repeated until all resources hit an equilibrium price. However, step 2 gains additional complexity. If the price direction of a resource was downwards and it now has more demand then supply at the new price, the simulator will fulfill the request to the best of its ability and consider the price at equilibrium. Vise versa if the price direction of a resource was upwards.

### Limitations
We have yet to do actual integration tests to get results on the trading system. Also, it is highly unlikely we will ever have an ideal equilibrium price where all requests to buy and sell are met. Instead, the trading system will try to get as close as possible to an equilibrium price.

### Future Improvements
Currently we do a round robin when best trying to fulfill trade requests. In the future we can implement better systems for this.