#/employees/{id}
put_employee() {
    curl -v --location --request PUT 'localhost:5000/employees/1' \
    --header 'Content-Type: application/json' \
    --data '{
    "id": 1234,
    "first_name": "Ola Updated",
    "last_name": "John Updated",
    "department": "Engineering Updated",
    "salary": 400,
    "age": 220
    }'
}

put_employee