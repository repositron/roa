# Restaurant Server
## Order API

### POST new Order
#### Json body
* id is a uuid
* tableId is an integer
* item is a string

#### example
``` bash
curl --location 'http://localhost:3000/orders' \
--header 'Content-Type: application/json' \
--data '{
"id": "40d6c3da-6b09-4982-857f-2881410a4a23",
"tableId": 46,
"item": "burger"
}'
```
### GET all orders for tableId
#### Example
```
curl --location 'http://localhost:3000/orders/55'
```

### Delete by id
Use the id to delete an order
#### Example
```
curl --location --request DELETE 'http://localhost:3000/orders/856b7ab3-5b91-4aa9-8411-af14a77839fc'
```
## Assumptions
* table id might be managed by another service, and client will first query this service.

## Todo
 * https
 * tokens in header for controlling access.