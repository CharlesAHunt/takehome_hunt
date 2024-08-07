
● Any necessary instructions for running your script

Currently, the script runs in memory, to emulate calling it repeatedly every hour and updating the stats you
can call monitor_currency_pairs(ds) in a loop and pass the result to the next invocation.
  
Run with Docker:
    
Run with Cargo:
   ``cargo run --package takehome_hunt --bin takehome_hunt``
    
● Any dependencies that need to be met
* To run with Docker, must have docker engine
* To run with Cargo, must install Cargo for the Rust ecosystem

● What you would do next to further improve it
* Use a real data store for the hourly results, instead of storing in memory
  * Etcd, DynamoDB, Redis
* Would likely not use Standard Deviation, which is more useful for normal distributions, currency valuation and stocks are not normal.  I would instead use a median absolute deviation.
* Error handling for JSON serialization of response
* I update the map each loop iteration, in a prod scenario I would likely do the same, then push that final data structure to persistent storage


● Other interesting checks you might implement to alert on market behaviour
* Alert for "stuck" prices, alerting for no price change over X days
* Missing symbols, symbols that previously existed that are no longer returned from the feed
* Alert for new symbols not in the previous feed
* Use MAD, Median Absolute Deviation

● Your approach to solving the task, and any issues you faced with implementation
* Decided to use the feeds API since it provided all the symbols and prices in a single API call
* Iterate around all values from the API, calculating the stats for each and updating the memory map
* Decided to use a library for stats calculation instead of 'reinventing the wheel'
* Assume that this script would persist the DataStore data to a persistent datastore or persistent volume in a prod scenario
