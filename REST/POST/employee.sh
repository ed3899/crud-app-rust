#/employees
post_employee() {
    curl --location 'localhost:5000/employees' \
    --header 'Content-Type: application/json' \
    --data '{
    "id": 1234,
    "first_name": "Ola",
    "last_name": "John",
    "department": "Engineering",
    "salary": 400,
    "age": 220
    }'
}

post_employee
