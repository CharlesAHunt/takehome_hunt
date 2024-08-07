###Any necessary instructions for running your script

The script tracks stats in memory, to emulate calling it repeatedly every hour and updating the stats I call
monitor_currency_pairs(ds) in a loop with a delay and pass the result to the next invocation.
Should see at least a couple alerts with a run during normal trading hours, if not, increase the delay.

Run with Docker:

`docker build -t takehome_hunt .`

`docker run takehome_hunt`


Run with Cargo:

   `cargo run --package takehome_hunt --bin takehome_hunt`
    
###Any dependencies that need to be met

* To run with Docker, must have Docker engine
* To run with Cargo, must install Cargo for the Rust ecosystem
  * https://doc.rust-lang.org/cargo/getting-started/installation.html

### What you would do next to further improve it
* Use a separate persistent data store for the hourly results, instead of storing in memory
  * Etcd, DynamoDB, Redis
* Run this script with a k8s job, perhaps with a persistent volume for data or an external store
* Handle the case of the job starting with no price changes for the first two runs then a price change of any amount will always trigger a std deviation for any small price change.  This case should likely be ignored. (i.e two prices in a row of 5, then a third run of 5.1 will trigger an alert)
* Would likely not use Standard Deviation, which is more useful for normal distributions, currency valuation and stocks are not normal.  I would instead use a median absolute deviation.
* Error handling for JSON serialization of response
* I update the map each loop iteration, in a prod scenario I would likely do the same, then push that final data structure to persistent storage
* Handle indexing the data per hour in the map so we are working with hourly data, not data from each run which may not be on an hourly basis
  * Could assume the job is running with an hourly cron and avoid that work

### Other interesting checks you might implement to alert on market behaviour
* Alert for "stuck" prices, alerting for no price change over X days
* Missing symbols, symbols that previously existed that are no longer returned from the feed
* Alert for new symbols not in the previous feed
* Use MAD, Median Absolute Deviation

### Your approach to solving the task, and any issues you faced with implementation
* Decided to use the feeds API since it provided all the symbols and prices in a single API call
* Iterate around all values from the API, calculating the stats for each and updating the memory map
* Add a delay of 1s between 4 invocations to emulate the script being called each hour
* Decided to use a library for stats calculation instead of 'reinventing the wheel'
* Assume that this script would persist the DataStore data to a persistent datastore or persistent volume in a prod scenario
* Assume there is a K8s cron job or something similar to trigger a run each hour
