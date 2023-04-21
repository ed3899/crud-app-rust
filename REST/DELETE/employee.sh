# /employee/{id}
delete_employee() {
    curl --location --request DELETE 'localhost:5000/employees/1'
}

delete_employee